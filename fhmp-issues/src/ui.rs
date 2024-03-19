use ratatui::{prelude::*, widgets::*};

use crate::issues::Issue;
use crate::app_state::AppState;

pub fn view(app: &mut AppState, frame: &mut Frame) {
    let rows = app.issues.all.iter()
        .map(issue_to_row)
        .collect::<Vec<_>>();

    let widths = [
        Constraint::Length(3),
        Constraint::Fill(1),
    ];

    let table = Table::new(rows, widths)
        .block(Block::default()
            .title("Issues")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(
        table,
        frame.size(),
        &mut app.table_state);
}

fn issue_to_row<'a>((k, v): (&usize, &anyhow::Result<Issue>)) -> Row<'a> {
    match v {
        Ok(i) => Row::new(vec![
            Cell::from(format!("{k}"))
                .style(Style::default()
                .fg(Color::LightBlue)),
            Cell::from(i.header.clone())
        ]),
        Err(e) => Row::new(vec![
            Cell::from(format!("{k}"))
                .style(Style::default()
                .fg(Color::Red)),
            Cell::from(e.to_string())
        ]),
    }
}
