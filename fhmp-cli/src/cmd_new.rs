use anyhow::Result;
use rand::distributions::{Alphanumeric, DistString};
use std::fs;
use std::io;

pub fn exec() -> Result<()> {
    loop {
        let dir = random_string(7);
        let res = fs::create_dir(&dir);
        match res {
            Ok(_) => {
                println!("directory created: ./{}", dir);
                fs::File::create(format!("./{dir}/readme.md"))?;
                fs::File::create(format!("./{dir}/reviews.txt"))?;
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