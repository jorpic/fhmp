use std::cmp;
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
use issues::read_issues_from;

fn main() -> Result<()> {
    let config = config::read_config()?;
    let (issues, _) = read_issues_from(&config.issues_path)?;
    let mut app = AppState {
        config,
        issues,
        table_state: Default::default(),
    };
    app.table_state.select(Some(0));

    tui::enter_alt_screen()?;

    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        tui::exit_alt_screen().expect("failed to exit alt screen");
        panic_hook(panic);
    }));

    let mut term = tui::new()?;
    let event_source =
        tui::EventSource::start_event_thread(Duration::from_millis(250));

    loop {
        term.draw(|frame| ui::view(&mut app, frame))?;

        if let Ok(ev) = event_source.receiver.recv() {
            match ev.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') => table_move(&mut app, 1),
                KeyCode::Char('k') => table_move(&mut app, -1),
                _ => {}
            }
        }
    }

    tui::exit_alt_screen()?;
    Ok(())
}

fn table_move(ts: &mut AppState, delta: isize) {
    let pos = ts.table_state.selected_mut().get_or_insert(0);
    let len = ts.issues.len();
    if delta > 0 {
        *pos = cmp::min(pos.saturating_add_signed(delta), len - 1);
    } else {
        *pos = cmp::max(pos.saturating_add_signed(delta), 0);
    }
}
