#![allow(dead_code, unused_imports)]
#![feature(box_patterns, specialization)]

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
