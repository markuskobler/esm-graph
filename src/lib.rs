#![allow(dead_code, unused_imports, bare_trait_objects)]
#![feature(box_patterns, specialization)] // TODO: find a way to move off nightly

use std::error::Error;

mod error;
mod parse;

pub struct Graph {
    entries: Vec<String>,
}

impl Graph {
    pub fn new(entries: Vec<String>) -> Graph {
        Graph { entries }
    }

    pub fn run(&self) -> Result<Vec<String>, error::Error> {
        println!("Scanning: {:?}", self.entries);

        let files = vec![];

        Ok(files)
    }
}
