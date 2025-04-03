use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Args {
    pub root: PathBuf,
    pub patterns: Vec<String>,
}