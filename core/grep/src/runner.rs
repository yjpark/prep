use ignore::{types::TypesBuilder, WalkBuilder};

use crate::{Args, Matched};

pub struct Runner {
}

impl Runner {
    pub fn run(&self, args: Args) -> Matched {
        let types = TypesBuilder::new()
            .add_defaults()
            .select(&args.language)
            .build()
            .unwrap();
        let walker = WalkBuilder::new(args.root)
            .types(types)
            .build();
        for result in walker {
            if result.is_err() { continue; }
            let entry = result.unwrap();
            if entry.file_type().map(|x| x.is_dir()).unwrap_or(true) {
                continue;
            }
            println!("Walk: {entry:?}");
        }
        todo!()
    }
}