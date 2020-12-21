pub trait Markdown {
  fn markdown(&self, repo: &Option<String>) -> String;
}
