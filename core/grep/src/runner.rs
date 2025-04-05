use grep::{regex::RegexMatcherBuilder, searcher::SearcherBuilder};
use ignore::{types::TypesBuilder, WalkBuilder};

use crate::{Args, Matched, MatchedFile};

pub struct Runner {
}

impl Runner {
    pub fn run(&self, args: Args) -> Matched {
        let types = TypesBuilder::new()
            .add_defaults()
            .select(&args.language)
            .build()
            .unwrap();
        let walker = WalkBuilder::new(args.root.clone())
            .types(types)
            .build();

        let matcher = RegexMatcherBuilder::new()
            .fixed_strings(true)
            .build_many(&args.patterns)
            .unwrap();
        let mut searcher = SearcherBuilder::new()
            .line_number(true)
            .build();

        let mut matched = Matched{ args: args.clone(), files: vec![] };

        for result in walker {
            if result.is_err() { continue; }
            let entry = result.unwrap();
            if entry.file_type().map(|x| x.is_dir()).unwrap_or(true) {
                continue;
            }
            let mut matched_file = MatchedFile {
                path: entry.path().to_path_buf(),
                lines: vec![],
            };
            let result = searcher.search_path(&matcher, entry.path(), &mut matched_file);
            if result.is_ok() {
                if matched_file.lines.len() > 0 {
                    matched.files.push(matched_file);
                }
            }
        }
        matched
    }
}