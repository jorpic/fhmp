use ratatui::{prelude::*, widgets::*};

use crate::app_state::AppState;

pub fn view(app: &AppState, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .block(
                Block::default()
                    .title("Titty!")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .black()
            .on_blue(),
        frame.size(),
    );
}
