//! Terminal-native graphics: Braille gives a 2×4 dot grid per character cell.
//! Canvas output becomes ordinary styled lines so it stays in the notebook's
//! scrolling, clipping and headless-snapshot pipeline.
use rasqp::Circuit;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        Widget,
        canvas::{Canvas, Context, Line as Stroke, Points},
    },
};

const GRID: Color = Color::Rgb(54, 75, 96);
const REFERENCE: Color = Color::Rgb(86, 200, 194);
const OBSERVED: Color = Color::Rgb(242, 188, 115);
const INK: Color = Color::Rgb(16, 30, 46);
const BLUE: Color = Color::Rgb(132, 170, 255);

/// Upgrade legacy text illustrations without changing prose or raw evidence.
pub fn typeset(text: &str) -> String {
    let text = text
        .replace("|0>", "∣0⟩")
        .replace("|1>", "∣1⟩")
        .replace("|+>", "∣+⟩")
        .replace("<->", "◀─▶")
        .replace("-->", "──▶")
        .replace("<--", "◀──")
        .replace("->", "─▶")
        .replace('|', "│")
        .replace('#', "█");
    let characters: Vec<_> = text.chars().collect();
    characters
        .iter()
        .enumerate()
        .map(|(index, character)| {
            if *character == '-'
                && (index > 0 && characters[index - 1] == '-'
                    || characters.get(index + 1) == Some(&'-'))
            {
                '─'
            } else {
                *character
            }
        })
        .collect()
}

fn canvas<F>(width: u16, height: u16, x: [f64; 2], y: [f64; 2], paint: F) -> Vec<Line<'static>>
where
    F: Fn(&mut Context<'_>),
{
    let area = Rect::new(0, 0, width, height);
    let mut buffer = Buffer::empty(area);
    Canvas::default()
        .marker(Marker::Braille)
        .background_color(INK)
        .x_bounds(x)
        .y_bounds(y)
        .paint(paint)
        .render(area, &mut buffer);
    buffer
        .content
        .chunks(width as usize)
        .map(|row| {
            Line::from(
                row.iter()
                    .map(|cell| Span::styled(cell.symbol().to_owned(), cell.style()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

/// A labeled illustration of |+>, not a reconstructed or noisy multi-qubit state.
pub fn bloch(width: u16) -> Vec<Line<'static>> {
    let width = width.clamp(20, 46);
    let height = 12;
    // Braille has twice as many vertical dots as horizontal dots in each cell.
    // Account for that aspect ratio so the sphere is round in a typical terminal.
    let half_x = 1.35 * f64::from(width) / (2.0 * f64::from(height));
    canvas(width, height, [-half_x, half_x], [-1.35, 1.35], |ctx| {
        for (radius_x, radius_y) in [(1.0, 1.0), (1.0, 0.26), (0.34, 1.0)] {
            let points: Vec<_> = (0..=360)
                .map(|i| {
                    let angle = (i as f64).to_radians();
                    (radius_x * angle.cos(), radius_y * angle.sin())
                })
                .collect();
            ctx.draw(&Points {
                coords: &points,
                color: BLUE,
            });
        }
        ctx.draw(&Stroke::new(0.0, -1.0, 0.0, 1.0, GRID));
        ctx.draw(&Stroke::new(-0.9, 0.0, 1.15, 0.0, GRID));
        ctx.draw(&Stroke::new(0.55, 0.22, -0.8, -0.33, GRID));
        ctx.layer();
        // |+> lies on the positive X axis of the single-qubit Bloch sphere.
        ctx.draw(&Stroke::new(0.0, 0.0, 1.0, 0.0, REFERENCE));
        ctx.draw(&Stroke::new(0.76, 0.12, 1.0, 0.0, REFERENCE));
        ctx.draw(&Stroke::new(0.76, -0.12, 1.0, 0.0, REFERENCE));
        ctx.print(-0.12, 1.24, Span::styled("|0⟩", Style::default().fg(BLUE)));
        ctx.print(-0.12, -1.12, Span::styled("|1⟩", Style::default().fg(BLUE)));
        ctx.print(
            1.08,
            0.09,
            Span::styled("X", Style::default().fg(REFERENCE)),
        );
        ctx.print(-0.9, -0.48, Span::styled("Y", Style::default().fg(BLUE)));
    })
}

/// Paired discrete probability stems; positions and heights come from SDK results.
/// No interpolation between outcomes or invented distribution smoothing.
pub fn distribution(reference: &[f64], observed: &[f64], width: u16) -> Vec<Line<'static>> {
    if reference.is_empty() || reference.len() != observed.len() {
        return Vec::new();
    }
    let count = reference.len();
    let bits = count.ilog2() as usize;
    canvas(
        width.clamp(20, 72),
        12,
        [-0.6, count as f64 - 0.4],
        [-0.32, 1.12],
        |ctx| {
            for y in [0.0, 0.5, 1.0] {
                ctx.draw(&Stroke::new(-0.4, y, count as f64 - 0.6, y, GRID));
            }
            ctx.layer();
            for (index, (expected, actual)) in reference.iter().zip(observed).enumerate() {
                for (offset, value, color) in
                    [(-0.12, *expected, REFERENCE), (0.12, *actual, OBSERVED)]
                {
                    let x = index as f64 + offset;
                    ctx.draw(&Stroke::new(x, 0.0, x, value.clamp(0.0, 1.0), color));
                    ctx.draw(&Stroke::new(x - 0.07, value, x + 0.07, value, color));
                }
                ctx.print(
                    index as f64 - 0.17,
                    -0.24,
                    Span::styled(format!("{index:0bits$b}"), Style::default().fg(BLUE)),
                );
            }
        },
    )
}

/// Eighth-block fill preserves sub-cell differences in probability bars.
pub fn fractional_bar(value: f64, width: usize) -> (String, usize) {
    let value = if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let eighths = (value * width as f64 * 8.0).round() as usize;
    let full = eighths / 8;
    let fraction = eighths % 8;
    let mut text = "█".repeat(full);
    if fraction > 0 {
        text.push([' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉'][fraction]);
    }
    (text, width.saturating_sub(full + usize::from(fraction > 0)))
}

/// Box-drawn quantum wires with full gate boxes and continuous CX links.
/// Split long circuits into panels rather than wrapping an electrical connection.
pub fn circuit(circuit: &Circuit, width: u16) -> String {
    const SLOT: usize = 7;
    let per_panel = (width.saturating_sub(10) as usize / SLOT).max(1);
    let panels = circuit.gates.len().div_ceil(per_panel).max(1);
    let mut rows = Vec::new();
    for panel in 0..panels {
        if panel > 0 {
            rows.push(String::new());
        }
        let start = panel * per_panel;
        let end = (start + per_panel).min(circuit.gates.len());
        if panels > 1 {
            rows.push(format!(
                "Gates {}–{} of {}",
                start + 1,
                end,
                circuit.gates.len()
            ));
        }
        for q in 0..circuit.width {
            let mut top = "        ".to_owned();
            let mut middle = if panel == 0 {
                format!("q{q:<2} |0⟩ ")
            } else {
                format!("q{q:<2}     ")
            };
            let mut bottom = "        ".to_owned();
            for gate in &circuit.gates[start..end] {
                if gate.name == "cx" {
                    let control = gate.qubits[0];
                    let target = gate.qubits[1];
                    let first = control.min(target);
                    let last = control.max(target);
                    top.push_str(if q > first && q <= last {
                        "   │   "
                    } else {
                        "       "
                    });
                    middle.push_str(if q == control {
                        "───●───"
                    } else if q == target {
                        "───⊕───"
                    } else if q > first && q < last {
                        "───┼───"
                    } else {
                        "───────"
                    });
                    bottom.push_str(if q >= first && q < last {
                        "   │   "
                    } else {
                        "       "
                    });
                } else if gate.qubits.contains(&q) {
                    top.push_str(" ┌───┐ ");
                    middle.push_str(&format!("─┤{:^3}├─", gate.name.to_uppercase()));
                    bottom.push_str(" └───┘ ");
                } else {
                    top.push_str("       ");
                    middle.push_str("───────");
                    bottom.push_str("       ");
                }
            }
            middle.push_str(if panel + 1 == panels {
                "─M"
            } else {
                "─▶"
            });
            rows.extend([
                top.trim_end().to_owned(),
                middle,
                bottom.trim_end().to_owned(),
            ]);
        }
    }
    rows.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rasqp::Gate;
    #[test]
    fn braille_canvas_preserves_size_and_subcell_shapes() {
        for width in [24, 32, 64] {
            let graphic = bloch(width);
            assert_eq!(graphic.len(), 12);
            assert!(graphic.iter().all(|line| line.width() <= width as usize));
            assert!(
                graphic
                    .iter()
                    .flat_map(|line| line.spans.iter())
                    .any(|span| span
                        .content
                        .chars()
                        .any(|c| ('\u{2801}'..='\u{28ff}').contains(&c)))
            );
            let plot = distribution(&[0.3, 0.7], &[0.5, 0.5], width);
            assert_eq!(plot.len(), 12);
            assert!(plot.iter().all(|line| line.width() <= width as usize));
        }
    }
    #[test]
    fn fractional_bars_show_eighths_and_preserve_total_width() {
        assert_eq!(fractional_bar(0.125, 1), ("▏".into(), 0));
        assert_eq!(fractional_bar(0.5, 1), ("▌".into(), 0));
        for value in [0.0, 0.001, 0.3, 0.5, 0.999, 1.0] {
            let (fill, empty) = fractional_bar(value, 17);
            assert_eq!(fill.chars().count() + empty, 17);
        }
        assert_eq!(fractional_bar(0.0, 3), (String::new(), 3));
        assert_eq!(fractional_bar(1.0, 3), ("███".into(), 0));
    }
    #[test]
    fn circuit_links_follow_gate_operands_and_do_not_wrap() {
        let c = Circuit::new(3, vec![Gate::h(0), Gate::cx(2, 0), Gate::x(1)]).unwrap();
        let text = circuit(&c, 31);
        assert!(text.lines().all(|line| line.chars().count() <= 31));
        let rows: Vec<_> = text.lines().collect();
        let column = 8 + 7 + 3;
        assert_eq!(rows[1].chars().nth(column), Some('⊕'));
        assert_eq!(rows[4].chars().nth(column), Some('┼'));
        assert_eq!(rows[7].chars().nth(column), Some('●'));
        for row in [2, 3, 5, 6] {
            assert_eq!(rows[row].chars().nth(column), Some('│'));
        }
        let text = circuit(&c, 24);
        assert!(text.contains("Gates 3–3 of 3"));
        assert!(text.lines().all(|line| line.chars().count() <= 24));
        let identity = circuit(&Circuit::identity(1).unwrap(), 24);
        assert!(identity.contains("|0⟩ ─M"));
    }
}
