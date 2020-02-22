use std::process::exit;

use esm_graph::Graph;

fn main() {
    let args = Args::parse();

    if args.entries.is_empty() {
        eprintln!("Missing files");
        exit(1);
    }

    // TODO: improve handling
    for entry in args.init().into_iter() {
        match entry {
            Ok(p) => {
                eprintln!("{}", p);
            }
            Err(err) => {
                eprintln!("Err: {}", err);
                exit(1);
            }
        }
    }
}

struct Args {
    entries: Vec<String>,
    root:    Option<String>,
}

impl Args {
    fn parse() -> Args {
        use clap::{crate_version, App, Arg};

        let args = App::new("ESM Graph")
            .version(crate_version!())
            .arg(Arg::with_name("root").long("root").takes_value(true))
            .arg(Arg::with_name("FILES"))
            .get_matches();

        Args {
            root:    args.value_of_lossy("root").map(String::from),
            entries: args.values_of_lossy("FILES").unwrap_or_default(),
        }
    }

    fn init(self) -> Graph {
        let mut graph = Graph::new();

        if let Some(ref root) = self.root {
            graph.root(root);
        }

        self.entries.iter().fold(&mut graph, Graph::add);

        graph
    }
}
