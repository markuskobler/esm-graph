use std::io;

use esm_graph::Graph;

#[test]
fn parse_example_js() -> Result<(), io::Error> {
    let mut graph = Graph::new();

    graph.root("tests/data").add("example.js");

    let imports: Vec<String> = graph.into_iter().map(|v| v.expect("failed to parse")).collect();

    assert_eq!(vec!["\"example.js\""], imports);

    Ok(())
}
