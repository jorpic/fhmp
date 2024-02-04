use std::io::{stdout, Stdout};
use std::time::Duration;
use std::{sync, thread};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyEvent},
    terminal, ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};

pub fn new() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

pub fn enter_alt_screen() -> Result<()> {
    stdout().execute(terminal::EnterAlternateScreen)?;
    Ok(terminal::enable_raw_mode()?)
}

pub fn exit_alt_screen() -> Result<()> {
    stdout().execute(terminal::LeaveAlternateScreen)?;
    Ok(terminal::disable_raw_mode()?)
}

pub struct EventSource {
    pub receiver: sync::mpsc::Receiver<KeyEvent>,
}

impl EventSource {
    // FIXME: handle unwraps?
    pub fn start_event_thread(poll_time: Duration) -> Self {
        let (sender, receiver) = sync::mpsc::channel();
        thread::spawn(move || loop {
            if !event::poll(poll_time).unwrap() {
                continue;
            }
            let Event::Key(e) = event::read().unwrap() else {
                continue;
            };
            if e.kind == event::KeyEventKind::Press {
                sender.send(e).unwrap();
            }
        });
        EventSource { receiver }
    }
}
