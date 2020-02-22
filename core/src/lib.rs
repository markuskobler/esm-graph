#![allow(dead_code, unused_imports, bare_trait_objects)]
#![feature(box_patterns, specialization)] // TODO: find a way to move off nightly

use std::{
    fmt,
    path::{Path, PathBuf},
};

use log::*;

mod error;
mod imports;

use self::error::Error;

pub struct Graph {
    entries: Vec<PathBuf>,
    opts:    GraphOptions,
}

struct GraphOptions {
    root: Option<PathBuf>,
    // TODO: output?
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            entries: vec![],
            opts:    GraphOptions { root: None },
        }
    }

    pub fn add<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        let path = p.as_ref();
        debug!("graph: entry {}", path.to_string_lossy());
        self.entries.push(path.to_path_buf());
        self
    }

    pub fn root<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        let path = p.as_ref();
        debug!("graph: root {}", path.to_string_lossy());
        self.opts.root = Some(path.to_path_buf());
        self
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph::new()
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Graph").field("entries", &self.entries).finish()
    }
}

impl IntoIterator for Graph {
    type IntoIter = IntoIter;
    type Item = Result<String, Error>;

    fn into_iter(self) -> IntoIter {
        IntoIter {
            opts:    self.opts,
            entries: self.entries,
            imports: vec![],
            parser:  imports::Parser::default(),
        }
    }
}

pub struct IntoIter {
    opts:    GraphOptions,
    entries: Vec<PathBuf>,
    imports: Vec<String>, // TODO: convert to a richer type
    parser:  imports::Parser,
}

impl Iterator for IntoIter {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Result<String, Error>> {
        while !self.entries.is_empty() {
            if let Some(ref path) = self.entries.pop() {
                // TODO: implement!

                match self.parser.parse(path.clone()) {
                    Ok(inputs) => {
                        println!(">>>> {:?}", inputs);
                    }
                    Err(err) => return Some(Err(err)),
                }

                return Some(Ok(format!("{:?}", path)));
            }
            return None;
        }

        None
    }
}
