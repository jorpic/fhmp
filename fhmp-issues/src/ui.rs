use ratatui::{prelude::*, widgets::*};

use crate::issues::Issue;
use crate::app_state::AppState;

pub fn view(app: &AppState, frame: &mut Frame) {
    let mut table_state = TableState::default();

    let rows = app.issues.all.iter()
        .map(issue_to_row)
        .collect::<Vec<_>>();

    table_state.select(Some(1));

    let widths = [Constraint::Length(3), Constraint::Percentage(40), Constraint::Percentage(10)];
    let table = Table::new(rows, widths)
        .block(Block::default()
            .title("Issues")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));
    frame.render_stateful_widget(table, frame.size(), &mut table_state);
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
