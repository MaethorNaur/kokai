use crate::git::Git;
use crate::parser::ConventionalCommit;
use std::convert::TryFrom;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "changelog", author, about)]
pub struct Changelog {
  /// Path to the git repository
  #[structopt(default_value = ".")]
  pub repository: String,
  /// Create a release changelog from previous tag until this one.
  #[structopt(long = "tag")]
  pub tag: Option<String>,
}

impl Changelog {
  pub fn exec(&self) {
    if let Some(tag) = &self.tag {
      for c in Git::new(&self.repository).get_all_commits_before(&tag) {
        if let Ok(c) = ConventionalCommit::try_from(c) {
          println!("{}", c);
        }
      }
    }
  }
}