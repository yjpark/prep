use std::{path::PathBuf, process::ExitCode};

use edger_prep_grep::{Runner, Args};

fn main() -> ExitCode {

    let args = Args {
        root: PathBuf::from("."),
        language: "rust".to_string(),
        patterns: vec![
            "test".to_string(),
            "grep".to_string(),
        ],
    };

    let runner = Runner{};

    let matched = runner.run(args);
    println!("{matched:#?}");

    ExitCode::SUCCESS
}