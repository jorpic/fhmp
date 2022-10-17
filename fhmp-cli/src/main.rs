use std::env;
use anyhow::Result;

mod config;
mod note;
mod db;
mod cmd_add;
mod cmd_dump;
mod cmd_review;

fn help() -> Result<()> {
    println!("Usage:");
    println!("\tfhmp add − read notes in YAML format from stdin.");
    println!("\tfhmp review [tags*] − review matching notes from DB.");
    anyhow::bail!("Invalid arguments.");
}

fn main() -> Result<()> {
    ctrlc::set_handler(|| {
        let term = dialoguer::console::Term::stderr();
        let _res = term.show_cursor();
        std::process::exit(0);
    })?;

    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, cmd, more_args @ ..] => match &cmd[..] {
            "add" if more_args.is_empty() => cmd_add::exec(),
            "dump" if more_args.is_empty() => cmd_dump::exec(),
            "review" => cmd_review::exec(more_args),
            _     => help(),
        }
        _ => help()
    }
}
