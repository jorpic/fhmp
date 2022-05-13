use std::env;
use anyhow::Result;

mod config;
mod db;
mod cmd_add;
use cmd_add::cmd_add;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_, cmd] => match cmd.as_str() {
            "add" => cmd_add(),
            "rnd" => cmd_rnd(),
            "dump" => cmd_dump(),
            _ => anyhow::bail!("Invalid command '{}'", cmd),
        },
        _ => {
            println!("Usage:");
            println!("\tfhmp add − read YAML from stdin.");
            println!("\tfhmp rnd − review random note from DB.");
            anyhow::bail!("Invalid arguments.");
        }
    }
}

fn cmd_rnd() -> Result<()> {
    Ok(())
}

fn cmd_dump() -> Result<()> {
    Ok(())
}
