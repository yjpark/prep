use std::{path::PathBuf, process::ExitCode};

use edger_prep_grep::{Runner, Args};

fn main() -> ExitCode {

    let args = Args {
        root: PathBuf::from("."),
        language: "rust".to_string(),
        patterns: vec![],
    };

    let runner = Runner{};

    runner.run(args);

    ExitCode::SUCCESS
}