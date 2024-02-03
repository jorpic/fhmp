use anyhow::Result;

mod config;

fn main() -> Result<()> {
    let conf = config::read_config()?;
    Ok(())
}
