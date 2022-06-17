use std::env;
use anyhow::Result;

mod config;
mod db;
mod cmd_add;
use cmd_add::cmd_add;
mod cmd_review;
use cmd_review::cmd_review;

fn help() -> Result<()> {
    println!("Usage:");
    println!("\tfhmp add − read notes in YAML format from stdin.");
    println!("\tfhmp review [tags*] − review matching notes from DB.");
    anyhow::bail!("Invalid arguments.");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, cmd, more_args @ ..] => match &cmd[..] {
            "add" if more_args.is_empty() => cmd_add(),
            "review" => cmd_review(more_args),
            _     => help(),
        }
        _ => help()
    }
}
