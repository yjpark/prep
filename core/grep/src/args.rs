#[derive(Clone, Debug)]
pub struct Args {
    pub root: AsRef<Path>,
    pub patterns: Vec<String>,
}