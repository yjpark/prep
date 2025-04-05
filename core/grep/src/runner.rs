use ignore::WalkBuilder;

use crate::{Args, Matched};

pub struct Runner {
}

impl Runner {
    pub fn run(args: Args) -> Matched {
        let walker = WalkBuilder::new(args.root).build();
        for result in walker {
            println!("Walk: {result:?}");
        }
        todo!()
    }
}