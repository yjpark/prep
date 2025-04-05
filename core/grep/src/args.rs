use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Args {
    pub root: PathBuf,
    pub language: String,
    pub patterns: Vec<String>,
}
