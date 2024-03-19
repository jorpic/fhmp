use ratatui::{prelude::*, widgets::*};

use crate::app_state::AppState;
use crate::issues::Issue;

pub fn view(app: &mut AppState, frame: &mut Frame) {
    let (issues_frame, details_frame) = {
        let layout = if frame.size().width > frame.size().height * 2 {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Fill(1)])
                .split(frame.size())
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(40), Constraint::Fill(1)])
                .split(frame.size())
        };
        (layout[0], layout[1])
    };

    let rows = app.issues.iter().map(issue_to_row).collect::<Vec<_>>();

    let widths = [Constraint::Length(3), Constraint::Fill(1)];

    let table = Table::new(rows, widths)
        .block(
            Block::default()
                .title("Issues")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(table, issues_frame, &mut app.table_state);

    let issue_details = if let Some(i) = app.table_state.selected() {
        &app.issues[i].body
    } else {
        ""
    };

    frame.render_widget(
        Paragraph::new(issue_details)
            .block(
                Block::new()
                    .title("Details")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .wrap(Wrap { trim: true }),
        details_frame,
    );
}

fn issue_to_row(issue: &Issue) -> Row<'_> {
    Row::new(vec![
        Cell::from(issue.id.to_string())
            .style(Style::default().fg(Color::LightBlue)),
        Cell::from(issue.header.as_str()),
    ])
}
