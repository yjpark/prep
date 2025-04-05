use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Args {
    pub root: PathBuf,
    pub extension: String,
    pub patterns: Vec<String>,
}