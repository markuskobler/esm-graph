#![allow(dead_code, unused_imports)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(specialization)]

// use neon::prelude::*;
// use neon_serde;
// use serde::Deserialize;

mod parse;

pub struct Graph {
    entries: Vec<String>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { entries: Vec::new() }
    }

    pub fn run(&self) -> Result<(), ()> {
        println!("Scanning...");

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn standalone_require() {
        assert_eq!(true, true);
    }
}
