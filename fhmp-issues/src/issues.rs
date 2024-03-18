use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::Path;

pub type IssueId = usize;

pub struct Issue {
    pub id: IssueId,
    pub header: String,
    pub body: String,
}

pub struct Issues {
    pub all: BTreeMap<IssueId, Result<Issue>>,
}

impl Issues {
    pub fn read_from(path: &Path) -> Result<Issues> {
        let issue_dirs =
            fs::read_dir(path).with_context(|| format!("Read issues from {:?}", path))?;

        let mut issues = BTreeMap::new();
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

            issues.insert(id, read_issue(id, &issue_path));
        }

        Ok(Issues { all: issues })
    }
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
