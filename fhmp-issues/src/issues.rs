use anyhow::{Context, Result};
use std::fs;
use std::io::Read;
use std::path::Path;

pub type IssueId = usize;

pub struct Issue {
    pub id: IssueId,
    pub header: String,
    pub body: String,
}

pub struct Error {
    pub id: IssueId,
    pub err: anyhow::Error,
}

pub type Issues = Vec<Issue>;
pub type Errors = Vec<Error>;

pub fn read_issues_from(path: &Path) -> Result<(Issues, Errors)> {
    let issue_dirs = fs::read_dir(path)
        .with_context(|| format!("Read issues from {:?}", path))?;

    let mut issues = Vec::new();
    let mut errors = Vec::new();

    for d in issue_dirs {
        let Ok(d) = d else {
            continue;
        };
        let issue_dir = d.path();
        if issue_dir.is_file() {
            continue;
        }
        let Some(id) = issue_dir.file_name() else {
            continue;
        };
        let Some(id) = id.to_str() else {
            continue;
        };
        let Ok(id) = IssueId::from_str_radix(id, 10) else {
            continue;
        };
        let issue_path = d.path().join("readme.md");

        match read_issue(id, &issue_path) {
            Ok(issue) => issues.push(issue),
            Err(err) => errors.push(Error { id, err }),
        }
    }

    Ok((issues, errors))
}

fn read_issue(id: IssueId, path: &Path) -> Result<Issue> {
    let mut body = String::new();
    fs::File::open(path)?.read_to_string(&mut body)?;
    let mut lines = body.lines();
    let header = lines
        .next()
        .context("Issue must have a header")?
        .trim_start_matches('#')
        .trim_start_matches(char::is_whitespace)
        .to_string();
    Ok(Issue { id, header, body })
}
