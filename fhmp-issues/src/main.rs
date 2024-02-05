use std::panic;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::KeyCode;

mod app_state;
mod config;
mod issues;
mod tui;
mod ui;

use app_state::AppState;
use issues::Issues;

fn main() -> Result<()> {
    let config = config::read_config()?;
    let issues = Issues::read_from(&config.issues_path)?;
    let app = AppState { config, issues };

    tui::enter_alt_screen()?;

    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        tui::exit_alt_screen().expect("failed to exit alt screen");
        panic_hook(panic);
    }));

    let mut term = tui::new()?;
    let event_source = tui::EventSource::start_event_thread(Duration::from_millis(250));

    loop {
        term.draw(|frame| ui::view(&app, frame))?;

        if let Ok(ev) = event_source.receiver.recv() {
            match ev.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    tui::exit_alt_screen()?;
    Ok(())
}
