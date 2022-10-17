use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::config::read_config;
use crate::note::{DbNote, NoteData};
use crate::db::{init_schema, select_notes_for_review};

pub enum ReviewResult {
    Easy, Hard, Again
}

fn get_review_result() -> Result<Option<ReviewResult>> {
    let theme = ColorfulTheme::default();
    let res = FuzzySelect::with_theme(&theme)
        .default(0)
        .item("0 Skip")  // don't save any result
        .item("1 Easy")  // increase delay
        .item("2 Hard")  // decrease delay
        .item("3 Again") // delay for 5 minutes
        .interact()?;

    Ok(match res {
        1 => Some(ReviewResult::Easy),
        2 => Some(ReviewResult::Hard),
        3 => Some(ReviewResult::Again),
        _ => None,
    })
}

fn review_note(note: &DbNote) -> Result<Option<ReviewResult>> {
    let theme = ColorfulTheme::default();
    println!("\n#{}", note.tags);
    match &note.data {
        NoteData::Text(txt) => {
            println!("{}", txt);
            get_review_result()
        },
        NoteData::Card(card) => {
            println!("{}", card[0]);

            let res = FuzzySelect::with_theme(&theme)
                .default(0)
                .item("1 Show the answer")
                .item("2 That was easy")
                .interact()?;

            if res == 1 {
                Ok(Some(ReviewResult::Easy))
            } else {
                for txt in &card[1..] {
                    println!("{}", txt);
                }
                get_review_result()
            }
        }
    }
}

pub fn exec(tags: &[String]) -> Result<()> {
    let cfg = read_config()
        .context("Reading config")?;
    let db = sqlite::open(&cfg.db_path)
        .context("Opening database file")?;
    init_schema(&db)
        .context("Initializing database schema")?;

    let notes = select_notes_for_review(&db)?;

    for note in notes.iter() {
        if let Some(res) = review_note(note)? {
            // FIXME: save result and schedule next review
        }
    }
    Ok(())
}
