//! Hit regions come from the rendered widgets, including scrolled sidebar rows.
use super::App;
use crossterm::event::{
    self, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use ratatui::layout::{Position, Rect};
use std::io;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Tab(usize),
    Topic(usize),
    Key(KeyCode),
}
#[derive(Default)]
pub struct MouseState {
    pub targets: Vec<(Rect, Action)>,
    pub notebook: Rect,
    pub topics: Rect,
    pub scrollbar: Option<Rect>,
    pub dragging: bool,
}
impl MouseState {
    pub fn reset_layout(&mut self) {
        self.targets.clear();
        self.notebook = Rect::default();
        self.topics = Rect::default();
        self.scrollbar = None;
    }
    pub fn target(&mut self, area: Rect, action: Action) {
        if !area.is_empty() {
            self.targets.push((area, action));
        }
    }
}

/// Always release terminal mouse reporting when the event loop returns or unwinds.
pub struct MouseCapture;
impl MouseCapture {
    pub fn enable() -> io::Result<Self> {
        let capture = Self;
        crossterm::execute!(io::stdout(), event::EnableMouseCapture)?;
        Ok(capture)
    }
}
impl Drop for MouseCapture {
    fn drop(&mut self) {
        let _ = crossterm::execute!(io::stdout(), event::DisableMouseCapture);
    }
}

impl App {
    pub fn mouse(&mut self, event: MouseEvent) -> bool {
        let point = Position::new(event.column, event.row);
        match event.kind {
            MouseEventKind::Up(MouseButton::Left) => self.mouse.dragging = false,
            MouseEventKind::Drag(MouseButton::Left) if self.mouse.dragging && !self.help => {
                self.drag_scrollbar(event.row);
            }
            MouseEventKind::ScrollUp | MouseEventKind::ScrollDown if !self.help => {
                let down = event.kind == MouseEventKind::ScrollDown;
                if self.mouse.topics.contains(point) {
                    let max = self.matches().len().saturating_sub(self.sidebar_rows);
                    self.sidebar_offset = if down {
                        self.sidebar_offset.saturating_add(3).min(max)
                    } else {
                        self.sidebar_offset.saturating_sub(3)
                    };
                } else if self.mouse.notebook.contains(point) {
                    self.scroll = if down {
                        self.scroll.saturating_add(3).min(self.max_scroll)
                    } else {
                        self.scroll.saturating_sub(3)
                    };
                }
            }
            MouseEventKind::Down(MouseButton::Left) => {
                if !self.help
                    && let Some(track) = self.mouse.scrollbar
                    && track.contains(point)
                {
                    if event.row == track.y {
                        self.scroll = self.scroll.saturating_sub(1);
                    } else if event.row + 1 == track.bottom() {
                        self.scroll = self.scroll.saturating_add(1).min(self.max_scroll);
                    } else {
                        self.mouse.dragging = true;
                        self.drag_scrollbar(event.row);
                    }
                    return false;
                }
                if let Some((_, action)) = self
                    .mouse
                    .targets
                    .iter()
                    .rev()
                    .find(|(area, _)| area.contains(point))
                {
                    let action = *action;
                    // Clicking a control acts on it even while the search field has focus.
                    self.searching = false;
                    match action {
                        Action::Tab(tab) => self.set_tab(tab),
                        Action::Topic(index) => self.select_topic(index),
                        Action::Key(code) => {
                            return self.key(KeyEvent::new(code, KeyModifiers::NONE));
                        }
                    }
                }
            }
            _ => {}
        }
        false
    }
    fn drag_scrollbar(&mut self, row: u16) {
        if let Some(track) = self.mouse.scrollbar {
            let start = track.y.saturating_add(1);
            let length = track.height.saturating_sub(3).max(1);
            let position = row.saturating_sub(start).min(length);
            self.scroll =
                (u32::from(self.max_scroll) * u32::from(position) / u32::from(length)) as u16;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    fn draw(app: &mut App, width: u16, height: u16) {
        Terminal::new(TestBackend::new(width, height))
            .unwrap()
            .draw(|frame| super::super::ui::draw(frame, app))
            .unwrap();
    }
    fn event(kind: MouseEventKind, x: u16, y: u16) -> MouseEvent {
        MouseEvent {
            kind,
            column: x,
            row: y,
            modifiers: KeyModifiers::NONE,
        }
    }
    fn click(app: &mut App, action: Action) -> bool {
        let (hit, _) = app
            .mouse
            .targets
            .iter()
            .find(|(_, a)| *a == action)
            .copied()
            .expect("visible enabled control");
        app.mouse(event(MouseEventKind::Down(MouseButton::Left), hit.x, hit.y))
    }
    #[test]
    fn clicks_use_rendered_tabs_topics_and_experiment_controls() {
        let mut app = App::new("missing.json".into());
        draw(&mut app, 120, 40);
        click(&mut app, Action::Tab(1));
        draw(&mut app, 120, 40);
        click(&mut app, Action::Topic(2));
        assert_eq!(app.config.demo, 2);
        app.result = Some(super::super::experiments::run(app.config).unwrap());
        draw(&mut app, 120, 40);
        click(&mut app, Action::Topic(2));
        assert!(
            app.result.is_some(),
            "Clicking the selected topic must retain its evidence"
        );
        click(&mut app, Action::Key(KeyCode::Char('b')));
        assert!(app.result.is_none());
        assert_eq!(app.config.probe, 1);
        draw(&mut app, 120, 40);
        assert!(
            !app.mouse
                .targets
                .iter()
                .any(|(_, a)| *a == Action::Key(KeyCode::Char('e'))),
            "Save must be disabled without current evidence"
        );
        click(&mut app, Action::Key(KeyCode::Enter));
        assert!(app.worker.is_some());
        draw(&mut app, 120, 40);
        click(&mut app, Action::Topic(1));
        assert_eq!(
            app.config.demo, 2,
            "A running experiment keeps its original configuration"
        );
    }
    #[test]
    fn wheel_and_drag_scroll_the_notebook_with_bounds() {
        let mut app = App::new("missing.json".into());
        draw(&mut app, 80, 24);
        assert!(app.max_scroll > 0);
        let area = app.mouse.notebook;
        app.mouse(event(MouseEventKind::ScrollDown, area.x + 2, area.y + 2));
        assert_eq!(app.scroll, 3);
        app.mouse(event(MouseEventKind::ScrollUp, area.x + 2, area.y + 2));
        assert_eq!(app.scroll, 0);
        let bar = app.mouse.scrollbar.unwrap();
        app.mouse(event(
            MouseEventKind::Down(MouseButton::Left),
            bar.x,
            bar.bottom() - 2,
        ));
        assert_eq!(app.scroll, app.max_scroll);
        assert!(app.mouse.dragging);
        app.mouse(event(MouseEventKind::Drag(MouseButton::Left), 0, 0));
        assert_eq!(app.scroll, 0);
        app.mouse(event(MouseEventKind::Drag(MouseButton::Left), 0, u16::MAX));
        assert_eq!(app.scroll, app.max_scroll);
        app.mouse(event(MouseEventKind::Up(MouseButton::Left), bar.x, bar.y));
        assert!(!app.mouse.dragging);
    }
    #[test]
    fn scrolled_and_filtered_sidebar_clicks_resolve_visible_items() {
        let mut app = App::new("missing.json".into());
        app.set_tab(3);
        draw(&mut app, 60, 18);
        let list = app.mouse.topics;
        app.mouse(event(MouseEventKind::ScrollDown, list.x, list.y));
        assert_eq!(app.sidebar_offset, 3);
        assert_eq!(
            app.selected[3], 0,
            "Wheel scrolling browses the list without changing the open topic"
        );
        draw(&mut app, 60, 18);
        app.mouse(event(
            MouseEventKind::Down(MouseButton::Left),
            list.x,
            list.y,
        ));
        assert_eq!(app.selected[3], 3);
        app.search = "calibrat".into();
        app.filter_changed();
        draw(&mut app, 60, 18);
        let index = app.matches()[1];
        click(&mut app, Action::Topic(index));
        assert_eq!(app.selected[3], index);
        // Resize and keyboard navigation reveal the selected topic again.
        app.search.clear();
        app.selected[3] = app.topics().len() - 1;
        app.reveal_topic();
        draw(&mut app, 80, 24);
        assert!(
            app.mouse
                .targets
                .iter()
                .any(|(_, a)| *a == Action::Topic(app.selected[3]))
        );
    }
    #[test]
    fn modal_and_resize_hide_background_hit_regions() {
        let mut app = App::new("missing.json".into());
        draw(&mut app, 120, 40);
        click(&mut app, Action::Key(KeyCode::Char('?')));
        draw(&mut app, 120, 40);
        assert_eq!(app.mouse.targets.len(), 1);
        app.mouse(event(MouseEventKind::ScrollDown, 40, 10));
        assert_eq!(app.scroll, 0);
        click(&mut app, Action::Key(KeyCode::Esc));
        assert!(!app.help);
        draw(&mut app, 10, 5);
        assert!(app.mouse.targets.is_empty());
        assert!(app.mouse.scrollbar.is_none());
        draw(&mut app, 120, 40);
        app.searching = true;
        assert!(
            click(&mut app, Action::Key(KeyCode::Char('q'))),
            "Quit is clickable even while search has focus"
        );
    }
    #[test]
    fn every_enabled_action_fits_the_compact_toolbar() {
        let mut app = App::new("missing.json".into());
        for tab in 0..5 {
            app.set_tab(tab);
            draw(&mut app, 60, 18);
            for key in [KeyCode::Char('?'), KeyCode::Char('q'), KeyCode::Enter] {
                assert!(
                    app.mouse
                        .targets
                        .iter()
                        .any(|(_, a)| *a == Action::Key(key)),
                    "Missing {key:?} control on tab {tab}"
                );
            }
        }
    }
}
