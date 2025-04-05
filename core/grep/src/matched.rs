use std::path::PathBuf;

use grep::searcher::Sink;
use snafu::Whatever;

use crate::Args;

#[derive(Clone, Debug)]
pub struct Matched {
    pub args: Args,
    pub files: Vec<MatchedFile>,
}

#[derive(Clone, Debug)]
pub struct MatchedFile {
    pub path: PathBuf,
    pub lines: Vec<MatchedLine>,
}

#[derive(Clone, Debug)]
pub struct MatchedLine {
    //pub pattern: u8,
    pub line: u64,
    pub text: String,
}

impl Sink for MatchedFile {
    type Error = std::io::Error;

    fn matched(
        &mut self,
        _searcher: &grep::searcher::Searcher,
        mat: &grep::searcher::SinkMatch<'_>,
    ) -> Result<bool, Self::Error> {
        let line = mat.line_number().unwrap();
        let text = std::str::from_utf8(mat.bytes()).unwrap();
        let matched_line = MatchedLine {
            line,
            text: text.to_string(),
        };
        self.lines.push(matched_line);
        Ok(true)
    }
}