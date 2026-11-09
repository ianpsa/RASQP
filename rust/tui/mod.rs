mod content;
mod experiments;
mod mouse;
mod runner;
mod ui;
mod visuals;

use content::Topic;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use experiments::{Config, DemoResult};
use runner::{Evidence, LabRun};
use std::{
    io,
    path::PathBuf,
    sync::mpsc::{self, Receiver, TryRecvError},
    time::Duration,
};

pub const TABS: &[&str] = &["Learn", "Workbench", "Studies", "Code map", "Glossary"];
pub struct App {
    tab: usize,
    selected: [usize; 5],
    scroll: u16,
    max_scroll: u16,
    search: String,
    searching: bool,
    help: bool,
    raw: bool,
    config: Config,
    result: Option<DemoResult>,
    worker: Option<Receiver<Result<DemoResult, String>>>,
    evidence: Option<Evidence>,
    summary_path: PathBuf,
    lab: Option<LabRun>,
    unconfined: bool,
    status: String,
    last_log: String,
    last_directory: Option<PathBuf>,
    mouse: mouse::MouseState,
    sidebar_offset: usize,
    sidebar_rows: usize,
}
impl App {
    pub fn new(summary_path: PathBuf) -> Self {
        let evidence = Evidence::load(&summary_path);
        let status = match &evidence {
            Ok(_) => {
                "Recorded evidence loaded. Workbench runs are separate educational examples.".into()
            }
            Err(e) => format!("No evidence loaded: {e}. Studies can create a new native run."),
        };
        Self {
            tab: 0,
            selected: [0; 5],
            scroll: 0,
            max_scroll: 0,
            search: String::new(),
            searching: false,
            help: false,
            raw: false,
            config: Config::default(),
            result: None,
            worker: None,
            evidence: evidence.ok(),
            summary_path,
            lab: None,
            unconfined: false,
            status,
            last_log: String::new(),
            last_directory: None,
            mouse: mouse::MouseState::default(),
            sidebar_offset: 0,
            sidebar_rows: 1,
        }
    }
    fn topics(&self) -> &'static [Topic] {
        match self.tab {
            0 => content::LESSONS,
            1 => content::DEMOS,
            2 => content::STUDIES,
            3 => content::CODE_MAP,
            _ => content::GLOSSARY,
        }
    }
    fn matches(&self) -> Vec<usize> {
        let query = self.search.to_lowercase();
        self.topics()
            .iter()
            .enumerate()
            .filter(|(_, t)| {
                query.is_empty()
                    || t.title.to_lowercase().contains(&query)
                    || t.body.to_lowercase().contains(&query)
                    || t.source.to_lowercase().contains(&query)
            })
            .map(|(i, _)| i)
            .collect()
    }
    fn set_tab(&mut self, tab: usize) {
        self.tab = tab;
        self.scroll = 0;
        self.raw = false;
        self.search.clear();
        self.searching = false;
        self.sidebar_offset = 0;
        self.reveal_topic();
    }
    fn select_topic(&mut self, index: usize) {
        if self.tab == 1 && self.worker.is_some() {
            return;
        }
        if index >= self.topics().len() || index == self.selected[self.tab] {
            return;
        }
        self.selected[self.tab] = index;
        self.scroll = 0;
        if self.tab == 1 {
            self.config.demo = index;
            self.result = None;
        }
    }
    fn reveal_topic(&mut self) {
        if let Some(position) = self
            .matches()
            .iter()
            .position(|i| *i == self.selected[self.tab])
        {
            if position < self.sidebar_offset {
                self.sidebar_offset = position;
            } else if position >= self.sidebar_offset + self.sidebar_rows {
                self.sidebar_offset = position.saturating_sub(self.sidebar_rows.saturating_sub(1));
            }
        }
    }
    fn change_selection(&mut self, down: bool) {
        if self.tab == 1 && self.worker.is_some() {
            return;
        }
        let matches = self.matches();
        if matches.is_empty() {
            return;
        }
        let position = matches
            .iter()
            .position(|i| *i == self.selected[self.tab])
            .unwrap_or(0);
        let next = if down {
            (position + 1) % matches.len()
        } else {
            (position + matches.len() - 1) % matches.len()
        };
        self.select_topic(matches[next]);
        self.reveal_topic();
    }
    fn filter_changed(&mut self) {
        if let Some(index) = self.matches().first().copied() {
            self.selected[self.tab] = index;
            if self.tab == 1 {
                self.config.demo = index;
                self.result = None;
            }
        }
        self.scroll = 0;
        self.sidebar_offset = 0;
    }
    fn run_demo(&mut self, reroll: bool) {
        if self.worker.is_some() {
            return;
        }
        if reroll {
            self.config.seed += 10;
        }
        let config = self.config;
        let (sender, receiver) = mpsc::channel();
        self.worker = Some(receiver);
        self.result = None;
        self.status =
            "Running the SDK: 79 clean calibration + 64 clean + 64 candidate repetitions...".into();
        std::thread::spawn(move || {
            let result = experiments::run(config).map_err(|e| e.to_string());
            let _ = sender.send(result);
        });
    }
    fn start_lab(&mut self) {
        if self.lab.is_some() {
            self.status = "A study is already running; x cancels it.".into();
            return;
        }
        if !cfg!(target_os = "linux") && !self.unconfined {
            self.status = "Linux seccomp is unavailable here. Press u to explicitly choose an unconfined child, then Enter.".into();
            return;
        }
        let mode = content::STUDIES[self.selected[2]].title;
        match LabRun::start(&PathBuf::from("results/runs/tui"), mode, self.unconfined) {
            Ok(run) => {
                self.status = format!(
                    "Running {mode}. New evidence: {}. x cancels; quitting cancels the active child.",
                    run.directory.display()
                );
                self.last_directory = Some(run.directory.clone());
                self.last_log.clear();
                self.lab = Some(run);
            }
            Err(e) => self.status = format!("Could not start laboratory: {e}"),
        }
    }
    fn cancel_lab(&mut self) {
        if let Some(mut run) = self.lab.take() {
            self.last_log = run.log_tail();
            self.status = match run.cancel() {
                Ok(()) => format!(
                    "Study stopped. Partial files remain in {}. Previous loaded evidence is retained.",
                    run.directory.display()
                ),
                Err(e) => format!("Could not stop study: {e}"),
            };
        }
    }
    fn load_evidence(&mut self) {
        match Evidence::load(&self.summary_path) {
            Ok(e) => {
                self.evidence = Some(e);
                self.scroll = 0;
                self.status = "Native summary loaded; its recorded provenance and criteria are shown in Studies.".into();
            }
            Err(e) => {
                self.status = format!("Could not load evidence: {e}. Previous evidence retained.")
            }
        }
    }
    fn tick(&mut self) {
        if let Some(worker) = &self.worker {
            match worker.try_recv() {
                Ok(Ok(result)) => {
                    self.result = Some(result);
                    self.worker = None;
                    self.status = "Educational simulation finished. b changes probe; n noise; s shots; r uses a fresh seed; v shows raw evidence.".into();
                }
                Ok(Err(e)) => {
                    self.worker = None;
                    self.status = format!("Experiment failed: {e}");
                }
                Err(TryRecvError::Disconnected) => {
                    self.worker = None;
                    self.status = "Experiment worker stopped before returning evidence. Press Enter to retry.".into();
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        if let Some(run) = &mut self.lab {
            self.last_log = run.log_tail();
            match run.poll() {
                Ok(Some(success)) => {
                    let path = run.directory.join("native_summary.json");
                    let directory = run.directory.clone();
                    self.lab = None;
                    if success {
                        self.summary_path = path;
                        self.load_evidence();
                        if self
                            .evidence
                            .as_ref()
                            .is_some_and(|e| e.path == self.summary_path)
                        {
                            self.status = format!(
                                "Native study complete. Evidence saved to {}.",
                                directory.display()
                            );
                        }
                    } else {
                        self.status = format!(
                            "Laboratory failed. Inspect {}/runner.log; previous evidence retained.",
                            directory.display()
                        );
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    self.status = format!("Could not poll laboratory: {e}. x cancels the child.");
                }
            }
        }
    }
    /// Returns true when the user requests exit. LabRun::drop reaps active children.
    fn key(&mut self, key: KeyEvent) -> bool {
        if key.kind != KeyEventKind::Press {
            return false;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return true;
        }
        if self.help {
            match key.code {
                KeyCode::Char('q') => return true,
                KeyCode::Esc | KeyCode::Char('?') | KeyCode::Enter => self.help = false,
                _ => {}
            }
            return false;
        }
        if self.searching {
            match key.code {
                KeyCode::Esc => {
                    self.searching = false;
                    self.search.clear();
                }
                KeyCode::Enter => self.searching = false,
                KeyCode::Backspace => {
                    self.search.pop();
                    self.filter_changed();
                }
                KeyCode::Char(c) => {
                    self.search.push(c);
                    self.filter_changed();
                }
                _ => {}
            }
            return false;
        }
        match key.code {
            KeyCode::Char('q') => return true,
            KeyCode::Char('?') => self.help = true,
            KeyCode::Tab | KeyCode::Right => self.set_tab((self.tab + 1) % TABS.len()),
            KeyCode::BackTab | KeyCode::Left => {
                self.set_tab((self.tab + TABS.len() - 1) % TABS.len())
            }
            KeyCode::Char(c @ '1'..='5') => self.set_tab(c as usize - '1' as usize),
            KeyCode::Down | KeyCode::Char('j') => self.change_selection(true),
            KeyCode::Up | KeyCode::Char('k') => self.change_selection(false),
            KeyCode::PageDown | KeyCode::Char('d') => {
                self.scroll = self.scroll.saturating_add(8).min(self.max_scroll)
            }
            KeyCode::PageUp | KeyCode::Char('a') => self.scroll = self.scroll.saturating_sub(8),
            KeyCode::Home => self.scroll = 0,
            KeyCode::End => self.scroll = self.max_scroll,
            KeyCode::Char('/') if self.worker.is_none() || self.tab != 1 => {
                self.searching = true;
                self.search.clear();
            }
            KeyCode::Esc => {
                self.search.clear();
                self.raw = false;
                self.scroll = 0;
            }
            KeyCode::Char('v') if self.tab == 1 || self.tab == 2 => {
                self.raw = !self.raw;
                self.scroll = 0;
            }
            KeyCode::Enter if !self.matches().is_empty() => match self.tab {
                1 => self.run_demo(false),
                2 => self.start_lab(),
                _ => self.change_selection(true),
            },
            KeyCode::Char('r') if self.tab == 1 => self.run_demo(true),
            KeyCode::Char('e') if self.tab == 1 => {
                self.status = match &self.result {
                    Some(result) => match runner::save_demo(&result.raw) {
                        Ok(path) => format!("Educational evidence saved to {}.", path.display()),
                        Err(e) => format!("Could not save educational evidence: {e}"),
                    },
                    None => "Run the current experiment before saving evidence.".into(),
                };
            }
            KeyCode::Char('r') if self.tab == 2 => self.load_evidence(),
            KeyCode::Char('x') if self.tab == 2 => self.cancel_lab(),
            KeyCode::Char('u') if self.tab == 2 && self.lab.is_none() => {
                self.unconfined = !self.unconfined;
                self.status = if self.unconfined {
                    "Explicit unconfined mode selected for the next child run."
                } else {
                    "Linux seccomp selected for the next child run."
                }
                .into();
            }
            KeyCode::Char(c @ ('b' | 'n' | 's')) if self.tab == 1 && self.worker.is_none() => {
                match c {
                    'b' => self.config.probe = (self.config.probe + 1) % 4,
                    'n' => self.config.noise = (self.config.noise + 1) % 3,
                    _ => self.config.shots = (self.config.shots + 1) % 3,
                }
                self.result = None;
                self.scroll = 0;
                self.status = "Settings changed. Enter runs fresh clean calibration and evaluates the candidate.".into();
            }
            _ => {}
        }
        false
    }
}

pub fn run(summary: PathBuf) -> io::Result<()> {
    use io::IsTerminal;
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err(io::Error::other(
            "An interactive terminal is required. Use --snapshot learn for a plain-text preview.",
        ));
    }
    let mut app = App::new(summary);
    let mut terminal = ratatui::init();
    let result = (|| {
        let _mouse_capture = mouse::MouseCapture::enable()?;
        loop {
            app.tick();
            terminal.draw(|frame| ui::draw(frame, &mut app))?;
            if event::poll(Duration::from_millis(150))? {
                match event::read()? {
                    Event::Key(key) if app.key(key) => break,
                    Event::Mouse(event) if app.mouse(event) => break,
                    Event::Resize(_, _) => {
                        app.mouse = mouse::MouseState::default();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    })();
    ratatui::restore();
    result
}

pub fn snapshot(
    summary: PathBuf,
    tab: usize,
    demo: usize,
    width: u16,
    height: u16,
) -> io::Result<String> {
    use ratatui::{Terminal, backend::TestBackend};
    let mut app = App::new(summary);
    app.tab = tab;
    if tab == 1 {
        app.selected[1] = demo;
        app.config.demo = demo;
        app.result = Some(experiments::run(app.config).map_err(io::Error::other)?);
    }
    let mut terminal = Terminal::new(TestBackend::new(width, height))
        .expect("TestBackend initialization is infallible");
    terminal
        .draw(|frame| ui::draw(frame, &mut app))
        .expect("TestBackend drawing is infallible");
    let mut output = String::new();
    for row in terminal.backend().buffer().content.chunks(width as usize) {
        let line: String = row.iter().map(|cell| cell.symbol()).collect();
        output.push_str(line.trim_end());
        output.push('\n');
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn press(app: &mut App, code: KeyCode) {
        app.key(KeyEvent::new(code, KeyModifiers::NONE));
    }
    #[test]
    fn navigation_filtering_and_settings_do_not_relabel_old_results() {
        let mut app = App::new("missing.json".into());
        press(&mut app, KeyCode::Char('2'));
        app.result = Some(experiments::run(app.config).unwrap());
        press(&mut app, KeyCode::Char('b'));
        assert!(app.result.is_none());
        assert_eq!(app.config.probe, 1);
        press(&mut app, KeyCode::Down);
        assert_eq!(app.config.demo, 1);
        press(&mut app, KeyCode::Char('/'));
        for c in "phase".chars() {
            press(&mut app, KeyCode::Char(c));
        }
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.selected[1], 2);
        assert_eq!(app.config.demo, 2);
        press(&mut app, KeyCode::Char('3'));
        assert!(app.search.is_empty());
    }
    #[test]
    fn every_screen_renders_at_narrow_and_large_sizes() {
        for (width, height) in [(10, 5), (60, 18), (80, 24), (120, 40), (160, 50)] {
            for tab in 0..5 {
                let text = snapshot("missing.json".into(), tab, 0, width, height).unwrap();
                assert!(text.contains("RASQP") || text.contains("Resize"));
                if width >= 80 && height >= 24 {
                    assert!(text.contains("q quit"));
                    if tab == 1 {
                        assert!(text.contains("TVD"));
                    }
                }
            }
        }
    }
}
