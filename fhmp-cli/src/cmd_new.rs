use anyhow::Result;
use rand::distributions::{Alphanumeric, DistString};
use std::{env, fs, io, process};

pub fn exec() -> Result<()> {
    loop {
        let dir = random_string(7);
        let res = fs::create_dir(&dir);
        match res {
            Ok(_) => {
                let readme = format!("./{dir}/readme.md");
                let reviews = format!("./{dir}/reviews.txt");
                println!("directory created: ./{}", dir);
                fs::File::create(&readme)?;
                fs::File::create(&reviews)?;
                run_editor(readme)?;
                return Ok(());
            },
            Err(err)
                if err.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(err.into()),
        }
    }
}

fn random_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

fn run_editor(file: String) -> Result<process::ExitStatus> {
    let editor = env::var("EDITOR")?;
    process::Command::new(editor)
        .arg(file)
        .spawn()
        .and_then(|mut c| c.wait())
        .map_err(|e| e.into())
}