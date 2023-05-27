use anyhow::{Context, Result};
use rand::distributions::{Alphanumeric, DistString};
use std::{env, fs, io, path::{Path, PathBuf}, process};
use crate::config::read_config;

pub fn exec() -> Result<()> {
    let cfg = read_config()
        .context("Reading config")?;

    let dir = create_new_dir(&cfg.data_path)?;
    println!("directory created: ./{}", dir.display());

    let readme = dir.join("readme.md");
    fs::File::create(&readme)?;
    fs::File::create(&dir.join("reviews.txt"))?;
    run_editor(&readme)?;
    Ok(())
}

fn random_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

fn create_new_dir(path: &Path) -> Result<PathBuf> {
    loop {
        let dir = path.join(&random_string(7));
        if let Err(err) = fs::create_dir_all(&dir) {
            if err.kind() == io::ErrorKind::AlreadyExists {
                continue;
            }
            return Err(err.into());
        }
        return Ok(dir);
    }
}

fn run_editor(file: &Path) -> Result<process::ExitStatus> {
    let editor = env::var("EDITOR")?;
    process::Command::new(editor)
        .arg(file)
        .spawn()
        .and_then(|mut c| c.wait())
        .map_err(|e| e.into())
}