use ignore::WalkBuilder;

use crate::{Args, Matched};

pub struct Runner {
}

impl Runner {
    pub fn run(&self, args: Args) -> Matched {
        let walker = WalkBuilder::new(args.root).build();
        for result in walker {
            println!("Walk: {result:?}");
        }
        todo!()
    }
}