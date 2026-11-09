use super::{
    App, TABS, content,
    experiments::{self, DemoResult},
    mouse::Action,
    visuals,
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Wrap,
    },
};

const INK: Color = Color::Rgb(16, 30, 46);
const PAPER: Color = Color::Rgb(212, 224, 239);
const CYAN: Color = Color::Rgb(86, 200, 194);
const AMBER: Color = Color::Rgb(242, 188, 115);
const BLUE: Color = Color::Rgb(132, 170, 255);
const MUTED: Color = Color::Rgb(140, 155, 176);
const RULE: Color = Color::Rgb(54, 75, 96);

fn plain(text: impl Into<String>) -> Line<'static> {
    Line::from(text.into())
}
fn label(text: impl Into<String>) -> Line<'static> {
    Line::styled(
        text.into(),
        Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
    )
}
fn subdued(text: impl Into<String>) -> Line<'static> {
    Line::styled(text.into(), Style::default().fg(MUTED))
}
fn paragraphs(lines: &mut Vec<Line<'static>>, text: &str) {
    lines.extend(text.lines().map(|s| plain(s.to_owned())));
}
fn art(lines: &mut Vec<Line<'static>>, text: &str) {
    lines.extend(
        text.lines()
            .map(|s| Line::styled(s.to_owned(), Style::default().fg(BLUE))),
    );
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    app.mouse.reset_layout();
    frame.render_widget(
        Block::default().style(Style::default().bg(INK).fg(PAPER)),
        area,
    );
    if area.width < 60 || area.height < 18 {
        frame.render_widget(
            Paragraph::new("RASQP\nResize to at least 60 x 18.\nq quits; Ctrl-C exits.")
                .wrap(Wrap { trim: false }),
            area,
        );
        return;
    }
    let rows = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Min(8),
        Constraint::Length(2),
        Constraint::Length(3),
    ])
    .split(area);
    let header = Line::from(vec![
        Span::styled(
            " RASQP ",
            Style::default()
                .fg(INK)
                .bg(CYAN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "  Quantum assurance lab",
            Style::default().fg(PAPER).add_modifier(Modifier::BOLD),
        ),
        Span::styled("   /   local simulation", Style::default().fg(MUTED)),
    ]);
    frame.render_widget(
        Paragraph::new(vec![
            header,
            subdued(" Click tabs, topics and buttons. Use the wheel to scroll either panel."),
        ]),
        rows[0],
    );
    tabs(frame, app, rows[1]);
    let columns = Layout::horizontal([
        Constraint::Length(if area.width >= 110 { 31 } else { 24 }),
        Constraint::Min(20),
    ])
    .split(rows[2]);
    sidebar(frame, app, columns[0]);
    let lines = if app.matches().is_empty() {
        vec![
            label("No matching topics"),
            plain("Press / to search again, or Esc to clear the filter."),
        ]
    } else {
        match app.tab {
            1 => workbench(app, columns[1].width.saturating_sub(4)),
            2 => studies(app),
            _ => topic(app, columns[1].width.saturating_sub(4)),
        }
    };
    document(frame, app, columns[1], lines);
    let state = if let Some(run) = &app.lab {
        format!(
            "Study {} running ({:.1}s) | {} | x cancel",
            run.mode,
            run.started.elapsed().as_secs_f64(),
            if run.unconfined {
                "unconfined"
            } else {
                "seccomp"
            }
        )
    } else if app.worker.is_some() {
        "Workbench is computing clean calibration and candidate trials...".into()
    } else {
        app.status.clone()
    };
    frame.render_widget(
        Paragraph::new(state)
            .style(Style::default().fg(AMBER))
            .wrap(Wrap { trim: false }),
        rows[3],
    );
    controls(frame, app, rows[4]);
    if app.help {
        // A modal must not pass clicks or wheel events through to the notebook.
        app.mouse.reset_layout();
        help(frame, app, area);
    }
}

fn tabs(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(RULE)),
        area,
    );
    let mut x = area.x;
    for (index, title) in TABS.iter().enumerate() {
        let text = format!(" {} {title} ", index + 1);
        let width = (text.len() as u16).min(area.right().saturating_sub(x));
        let hit = Rect::new(x, area.y, width, 1);
        let style = if index == app.tab {
            Style::default()
                .fg(INK)
                .bg(CYAN)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(PAPER)
        };
        frame.render_widget(Paragraph::new(text).style(style), hit);
        app.mouse.target(hit, Action::Tab(index));
        x += width;
    }
}

fn controls(frame: &mut Frame, app: &mut App, area: Rect) {
    use crossterm::event::KeyCode::{Char, Enter, Esc};
    let ready = app.worker.is_none();
    let topics = !app.matches().is_empty();
    let mut buttons = match app.tab {
        1 => vec![
            ("Run".into(), Enter, ready && topics),
            (
                format!("Probe {}", app.config.probe_label()),
                Char('b'),
                ready,
            ),
            (
                format!("Noise {:.3}", app.config.noise_value()),
                Char('n'),
                ready,
            ),
            (
                format!("Shots {}", app.config.shot_count()),
                Char('s'),
                ready,
            ),
            ("New seed".into(), Char('r'), ready && topics),
            (
                if app.raw { "Notebook" } else { "JSON" }.into(),
                Char('v'),
                true,
            ),
            ("Save".into(), Char('e'), app.result.is_some()),
        ],
        2 => vec![
            ("Run".into(), Enter, app.lab.is_none() && topics),
            ("Cancel".into(), Char('x'), app.lab.is_some()),
            (
                if app.raw { "Notebook" } else { "JSON" }.into(),
                Char('v'),
                true,
            ),
            ("Reload".into(), Char('r'), true),
            (
                format!("Sandbox {}", if app.unconfined { "off" } else { "on" }),
                Char('u'),
                app.lab.is_none(),
            ),
        ],
        _ => vec![
            ("Next topic".into(), Enter, topics),
            ("Search".into(), Char('/'), true),
            ("Clear".into(), Esc, true),
        ],
    };
    buttons.extend([
        ("Help".into(), Char('?'), true),
        ("Quit".into(), Char('q'), true),
    ]);
    let mut x = area.x;
    let mut y = area.y;
    for (label, code, enabled) in buttons {
        let text = format!("[{label}]");
        let width = text.len() as u16;
        if x + width > area.right() {
            x = area.x;
            y += 1;
        }
        if y >= area.bottom().saturating_sub(1) {
            break;
        }
        let hit = Rect::new(x, y, width, 1);
        frame.render_widget(
            Paragraph::new(text).style(
                Style::default()
                    .fg(if enabled { CYAN } else { MUTED })
                    .bg(if enabled { RULE } else { INK }),
            ),
            hit,
        );
        if enabled {
            app.mouse.target(hit, Action::Key(code));
        }
        x += width + 1;
    }
    frame.render_widget(
        Paragraph::new(" Tab/1-5 page  ↑↓ topic  PgUp/PgDn scroll  ? help  q quit")
            .style(Style::default().fg(MUTED)),
        Rect::new(area.x, area.bottom().saturating_sub(1), area.width, 1),
    );
}

fn sidebar(frame: &mut Frame, app: &mut App, area: Rect) {
    let matches = app.matches();
    let title = if app.searching {
        format!(" Search: {}_ ", app.search)
    } else if app.search.is_empty() {
        format!(" {} topics ", matches.len())
    } else {
        format!(" /{} ", app.search)
    };
    let block = Block::default().title(title).borders(Borders::NONE);
    let inner = block.inner(area);
    app.sidebar_rows = inner.height as usize;
    app.sidebar_offset = app
        .sidebar_offset
        .min(matches.len().saturating_sub(app.sidebar_rows));
    app.mouse.topics = inner;
    let items: Vec<ListItem> = matches
        .iter()
        .map(|i| {
            let selected = *i == app.selected[app.tab];
            ListItem::new(format!(
                "{}{}",
                if selected { "› " } else { "  " },
                app.topics()[*i].title
            ))
            .style(if selected {
                Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            })
        })
        .collect();
    let mut state = ListState::default().with_offset(app.sidebar_offset);
    frame.render_stateful_widget(List::new(items).block(block), area, &mut state);
    app.sidebar_offset = state.offset();
    for (row, index) in matches
        .iter()
        .skip(app.sidebar_offset)
        .take(app.sidebar_rows)
        .enumerate()
    {
        app.mouse.target(
            Rect::new(inner.x, inner.y + row as u16, inner.width, 1),
            Action::Topic(*index),
        );
    }
}

fn document(frame: &mut Frame, app: &mut App, area: Rect, lines: Vec<Line<'static>>) {
    app.mouse.notebook = area;
    let block = Block::default()
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(RULE))
        .title(if app.raw {
            " Evidence / JSON "
        } else {
            " Notebook "
        });
    let inner = block.inner(area).inner(ratatui::layout::Margin::new(1, 0));
    frame.render_widget(block, area);
    let paragraph = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
    let count = paragraph.line_count(inner.width);
    app.max_scroll = count
        .saturating_sub(inner.height as usize)
        .min(u16::MAX as usize) as u16;
    app.scroll = app.scroll.min(app.max_scroll);
    frame.render_widget(paragraph.scroll((app.scroll, 0)), inner);
    if app.max_scroll > 0 {
        app.mouse.scrollbar = Some(Rect::new(
            area.right().saturating_sub(1),
            area.y,
            1,
            area.height,
        ));
        let mut state = ScrollbarState::new(count)
            .position(app.scroll as usize)
            .viewport_content_length(inner.height as usize);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut state,
        );
    }
}

fn topic(app: &App, width: u16) -> Vec<Line<'static>> {
    let topic = &app.topics()[app.selected[app.tab]];
    let mut lines = vec![label(topic.title), plain("")];
    if matches!(app.tab, 0 | 4) && app.selected[app.tab] == 1 {
        lines.push(label("A single qubit: the Bloch sphere"));
        lines.extend(visuals::bloch(width));
        lines.push(subdued(
            "Illustration: |+⟩ points along X; the poles are |0⟩ and |1⟩.",
        ));
        lines.push(subdued(
            "A coordinate picture of one qubit, not a measured state estimate.",
        ));
        lines.push(plain(""));
    } else if !topic.art.is_empty() {
        art(&mut lines, &visuals::typeset(topic.art));
        lines.push(plain(""));
    }
    paragraphs(&mut lines, topic.body);
    if app.tab == 0 && app.selected[0] == 1 {
        lines.push(plain(""));
        if let Ok(circuit) = rasqp::Circuit::new(2, vec![rasqp::Gate::h(0), rasqp::Gate::cx(0, 1)])
        {
            lines.push(label("Bell recipe: H then controlled X"));
            art(&mut lines, &visuals::circuit(&circuit, width));
            lines.push(subdued("● control  ⊕ conditional flip  M measurement"));
        }
    }
    lines.push(plain(""));
    lines.push(subdued(format!("Source: {}", topic.source)));
    lines
}

fn workbench(app: &App, width: u16) -> Vec<Line<'static>> {
    let config = app.config;
    let topic = &content::DEMOS[config.demo];
    let mut lines = vec![
        label(topic.title),
        plain(format!(
            "Probe {}   noise {:.3}   shots {}",
            config.probe_label(),
            config.noise_value(),
            config.shot_count()
        )),
        subdued(format!(
            "Seed {} | educational fixture | not a frozen protocol run",
            config.seed
        )),
        plain(""),
    ];
    if app.raw {
        if let Some(result) = &app.result {
            paragraphs(&mut lines, &result.raw);
        } else {
            lines.push(plain(
                "Press Enter to generate evidence for these settings.",
            ));
        }
        return lines;
    }
    if let Some(result) = &app.result {
        result_lines(&mut lines, result, config.demo, width);
        lines.push(plain(""));
        label_circuits(&mut lines, &result.approved, &result.candidate, width);
    } else {
        lines.push(Line::styled(
            if app.worker.is_some() {
                "Computing..."
            } else {
                "Enter runs this experiment"
            },
            Style::default().fg(AMBER),
        ));
        lines.push(plain(
            "79 clean calibration + 64 clean + 64 candidate repetitions.",
        ));
        lines.push(plain(
            "Reference: exact probabilities in the selected noise context.",
        ));
        if let Ok((approved, candidate)) = experiments::circuits(config.demo) {
            lines.push(plain(""));
            label_circuits(&mut lines, &approved, &candidate, width);
        }
    }
    lines.push(plain(""));
    label_explanation(&mut lines, topic);
    lines.push(plain(""));
    lines.push(subdued("The circuit art shows the raw recipe. The selected probe adds input preparation and measurement rotations around both recipes. q0 is the rightmost output bit."));
    lines
}

fn label_circuits(
    lines: &mut Vec<Line<'static>>,
    approved: &rasqp::Circuit,
    candidate: &rasqp::Circuit,
    width: u16,
) {
    lines.push(label("Approved recipe"));
    art(lines, &visuals::circuit(approved, width));
    lines.push(label("Candidate recipe"));
    art(lines, &visuals::circuit(candidate, width));
    lines.push(subdued("● control  ⊕ conditional flip  M measurement"));
    for (i, g) in candidate
        .gates
        .iter()
        .enumerate()
        .filter(|(_, g)| g.name.starts_with('r'))
    {
        lines.push(subdued(format!(
            "Gate {i}: {}({:.3} radians) on q{}",
            g.name, g.angle, g.qubits[0]
        )));
    }
}
fn label_explanation(lines: &mut Vec<Line<'static>>, topic: &content::Topic) {
    lines.push(label("What are we testing?"));
    paragraphs(lines, topic.body);
    lines.push(plain(""));
    lines.push(subdued(format!("SDK / registry: {}", topic.source)));
}
fn result_lines(lines: &mut Vec<Line<'static>>, r: &DemoResult, demo: usize, width: u16) {
    lines.push(Line::styled(
        format!(
            "First trial: {}  |  TVD {:.4} / tolerance {:.4}",
            if r.alarm {
                "behavioral alarm"
            } else {
                "no behavioral alarm"
            },
            r.score,
            r.threshold
        ),
        Style::default()
            .fg(if r.alarm { AMBER } else { CYAN })
            .add_modifier(Modifier::BOLD),
    ));
    lines.push(plain(format!(
        "Structural inspection: {:?} | ideal operators {}",
        r.inspection.decision,
        if r.equivalent {
            "equivalent"
        } else {
            "different"
        }
    )));
    lines.push(subdued(
        "A quiet behavioral check is not a guarded Accept decision.",
    ));
    lines.push(plain(""));
    let bar_width = width.saturating_sub(15).clamp(6, 52) as usize;
    for (i, (p, observed)) in r.reference.iter().zip(&r.observed).enumerate() {
        let bitstring = format!("{i:0width$b}", width = r.approved.width as usize);
        for (label, value, color) in [("ref", *p, CYAN), ("obs", *observed, AMBER)] {
            let (fill, empty) = visuals::fractional_bar(value, bar_width);
            lines.push(Line::from(vec![
                Span::styled(format!("{bitstring} {label} "), Style::default().fg(color)),
                Span::styled(fill, Style::default().fg(color)),
                Span::styled("░".repeat(empty), Style::default().fg(RULE)),
                Span::raw(format!(" {:5.1}%", value * 100.)),
            ]));
        }
    }
    lines.push(subdued(
        "ref = approved probability; obs = candidate frequency",
    ));
    lines.push(subdued(format!("First candidate counts: {:?}", r.counts)));
    lines.push(plain(""));
    for (name, rate) in [
        ("Clean alarms", &r.clean),
        (
            if matches!(demo, 0 | 5 | 6 | 7 | 8) {
                "Candidate alarms (benign)"
            } else {
                "Candidate alarms"
            },
            &r.changed,
        ),
    ] {
        lines.push(plain(format!(
            "{name}: {}/{} ({:.1}%)",
            rate.rejected,
            rate.n,
            rate.rate * 100.
        )));
        lines.push(subdued(format!(
            "  Wilson 95%: {:.1}% to {:.1}%",
            rate.wilson95[0] * 100.,
            rate.wilson95[1] * 100.
        )));
    }
    if let Some(contracts) = &r.contracts {
        lines.push(plain(""));
        lines.push(label("Actual signed-result gate outcomes"));
        for name in ["first_response", "replayed_response", "altered_counts"] {
            lines.push(plain(format!(
                "{name}: {} / counts {}",
                contracts[name]["decision"].as_str().unwrap_or("unknown"),
                if contracts[name]["accepted_counts"].is_null() {
                    "withheld"
                } else {
                    "released"
                }
            )));
        }
        lines.push(subdued(
            "Memory ledger + public demonstration key; no provider attestation.",
        ));
    }
    if let Some(runtime) = &r.runtime {
        lines.push(plain(""));
        lines.push(label("Actual guarded backend outcomes"));
        for name in ["approved_execution", "visible_edit"] {
            let outcome = &runtime[name];
            lines.push(plain(format!(
                "{name}: {} / counts {} / {} shots",
                outcome["decision"].as_str().unwrap_or("unknown"),
                if outcome["accepted_counts"].is_null() {
                    "withheld"
                } else {
                    "released"
                },
                outcome["shots_submitted"]
            )));
            lines.push(subdued(outcome["reason"].as_str().unwrap_or("")));
        }
    }
    lines.push(plain(""));
    lines.push(label("Distribution view · Braille canvas"));
    lines.push(subdued("Top / middle / bottom guide: 100% / 50% / 0%."));
    lines.extend(visuals::distribution(&r.reference, &r.observed, width));
    lines.push(Line::from(vec![
        Span::styled("Cyan: approved  ", Style::default().fg(CYAN)),
        Span::styled("Amber: observed", Style::default().fg(AMBER)),
    ]));
    lines.push(subdued(
        "Each pair of stems is one discrete outcome, not a continuous curve.",
    ));
    lines.push(plain(""));
    let interpretation = match demo {
        0 => "The candidate is a clean control. Any behavioral alarm is a false positive.",
        7 => {
            "The authentication and replay outcomes above are actual signed-result gate decisions. The behavioral rates are separate clean-control observations."
        }
        8 => {
            "The runtime outcomes above are actual guarded executions. The visible edit is withheld before backend submission; the behavioral rates describe the unchanged recipe."
        }
        2 | 3 if r.changed.rejected == 0 => {
            "This view missed the operator change in all 64 repetitions. Try b for a different input / measurement basis."
        }
        5 => {
            "The recipes are ideally equivalent. Extra physical gates can still change a noisy tally."
        }
        6 => {
            "The recipe is unchanged. These candidate alarms arise under benign simulated drift; they do not identify an attacker."
        }
        _ => {
            "Compare candidate alarms with clean alarms and their intervals. This is one bounded fixture, not a universal detection rate."
        }
    };
    paragraphs(lines, interpretation);
}

fn studies(app: &App) -> Vec<Line<'static>> {
    let topic = &content::STUDIES[app.selected[2]];
    let mode = topic.title;
    let mut lines = vec![
        label(format!("Native studies / {mode}")),
        subdued(format!(
            "Next run: {}",
            if app.unconfined {
                "explicit unconfined child"
            } else {
                "Linux seccomp child"
            }
        )),
        plain(""),
    ];
    if app.raw {
        if let Some(evidence) = &app.evidence {
            lines.push(subdued(format!("Loaded file: {}", evidence.path.display())));
            if let Some(raw) = evidence.raw.get(mode) {
                paragraphs(&mut lines, raw);
            } else {
                lines.push(plain(format!(
                    "This loaded run has no {mode} study. Enter runs it in a new directory."
                )));
            }
        } else {
            lines.push(plain(
                "No native evidence loaded. Enter runs the selected study.",
            ));
        }
        return lines;
    }
    if let Some(evidence) = &app.evidence {
        let s = &evidence.value;
        lines.push(label("Loaded evidence (separate from the next run)"));
        lines.push(plain(evidence.path.display().to_string()));
        lines.push(plain(format!(
            "Version {} | sandboxed {} | QPU shots {}",
            s["version"], s["sandboxed"], s["qpu_shots"]
        )));
        lines.push(plain(format!(
            "Simulation cost: {} shots / {} circuits / {} jobs",
            s["cost"]["shots"], s["cost"]["circuits"], s["cost"]["jobs"]
        )));
        lines.push(subdued(format!(
            "Protocol SHA-256: {}",
            s["protocol_sha256"].as_str().unwrap_or("unknown")
        )));
        if let Some(studies) = s["studies"].as_object() {
            lines.push(plain(format!(
                "Included: {}",
                studies.keys().cloned().collect::<Vec<_>>().join(", ")
            )));
            for (study, flag) in [
                ("extensions", "H41_pass"),
                ("extensions", "H42_pass"),
                ("context", "H44_pass"),
            ] {
                if mode != "all" && mode != study {
                    continue;
                }
                if let Some(passed) = studies.get(study).and_then(|v| v[flag].as_bool()) {
                    lines.push(Line::styled(
                        format!(
                            "{}: {} in this recorded native run",
                            flag.trim_end_matches("_pass"),
                            if passed {
                                "criterion passed"
                            } else {
                                "criterion failed"
                            }
                        ),
                        Style::default().fg(if passed { CYAN } else { AMBER }),
                    ));
                }
            }
            if mode != "all" && !studies.contains_key(mode) {
                lines.push(plain(format!(
                    "The loaded evidence does not include {mode}."
                )));
            }
        }
        lines.push(plain(""));
        lines.push(subdued("Recorded screening outcomes are scoped to these protocols. External studies have separate outcomes; these flags do not establish universal security or noisy-reference superiority."));
    } else {
        lines.push(plain(
            "No recorded native evidence loaded. Enter runs the selected study.",
        ));
    }
    if !app.last_log.is_empty() {
        lines.push(plain(""));
        lines.push(label("Latest child-process log"));
        paragraphs(&mut lines, &app.last_log);
    }
    if let Some(directory) = &app.last_directory {
        lines.push(subdued(format!("Latest output: {}", directory.display())));
    }
    lines.push(plain(""));
    label_explanation(&mut lines, topic);
    lines.push(plain(""));
    lines.push(label("Frozen native protocol"));
    if let Ok(protocol) = rasqp::lab::protocol::ProtocolRegistry::native() {
        let key = match mode {
            "extensions" => "extension_v03",
            "context" => "extension_v04",
            _ => mode,
        };
        if let Some(section) = protocol.get(key) {
            paragraphs(
                &mut lines,
                &serde_json::to_string_pretty(section).unwrap_or_default(),
            );
        }
        lines.push(subdued(protocol["status"].as_str().unwrap_or("")));
    }
    lines
}

fn help(frame: &mut Frame, app: &mut App, area: Rect) {
    let width = area.width.saturating_sub(6).min(86);
    let height = area.height.saturating_sub(2).min(29);
    let popup = Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    );
    frame.render_widget(Clear, popup);
    let text = "Click tabs, topics and buttons; wheel scrolls either panel.\nDrag the notebook scrollbar. [Close] dismisses this help.\n\nNavigate\n  Tab / Shift-Tab or 1-5: switch page\n  Up/Down or j/k: choose topic; Enter advances a lesson\n  PageUp/PageDown or a/d: scroll; Home/End: start/end\n  /: search titles, explanations and sources\n  Enter finishes search; Esc clears the filter / raw view\n\nWorkbench\n  Enter: calibrate and run; r: run with a new seed\n  b: preparation / basis; n: noise; s: shots; v: raw JSON; e: save\n  Settings changes clear old results. Probes wrap the recipe.\n\nStudies\n  Enter: run selected native study; x: stop the child\n  v: loaded summary JSON; r: reload; u: sandbox choice\n  Output: results/runs/tui/<unique run>/\n  Quitting stops an active lab; partial evidence is retained.\n\nq / Ctrl-C exits. ? / Esc / Enter closes this help.\nTerminal art and charts need no image protocol or QPU.";
    let text = if width < 70 || height < 24 {
        "Click tabs/topics/buttons; wheel scrolls.\nDrag the scrollbar; [Close] dismisses help.\n? / Esc / Enter: close help; q: quit\nTab / 1-5: page; arrows / jk: topic\nPgUp/PgDn or a/d: scroll; Home/End: jump\n/: search; Enter: finish; Esc: clear\n\nWorkbench: Enter run; r new seed\nb probe; n noise; s shots; v JSON; e save\n\nStudies: Enter run; x cancel\nv JSON; r reload; u sandbox choice\n\nQuit stops an active lab child."
    } else {
        text
    };
    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .style(Style::default().fg(PAPER).bg(INK))
            .block(
                Block::bordered()
                    .title(" How to use this notebook ")
                    .border_style(Style::default().fg(CYAN)),
            ),
        popup,
    );
    let close = Rect::new(popup.right().saturating_sub(9), popup.y, 7, 1);
    frame.render_widget(
        Paragraph::new("[Close]").style(Style::default().fg(INK).bg(CYAN)),
        close,
    );
    app.mouse
        .target(close, Action::Key(crossterm::event::KeyCode::Esc));
}
