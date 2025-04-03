use crate::Args;

#[derive(Clone, Debug)]
pub struct Matched {
    pub args: Args,
    pub files: Vec<MatchedFile>,
}

#[derive(Clone, Debug)]
pub struct MatchedFile {
    pub path: AsRef<Path>,
    pub lines: Vec<MatchedLine>,
}

#[derive(Clone, Debug)]
pub struct MatchedLine {
    pub pattern: u8,
    pub line: u32,
    pub text: String,
}