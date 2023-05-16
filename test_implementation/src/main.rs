mod default_graph;

use default_graph::{DefaultGraph};
use std::collections::HashMap;

fn main() {
    let mut delta: Vec<HashMap<char,usize>> = Vec::new();
    delta.push(HashMap::from([('a', 1), ('b', 3)]));
    delta.push(HashMap::from([('a', 2)]));
    delta.push(HashMap::from([('b', 5)]));
    delta.push(HashMap::from([('a', 4),('b', 3)]));
    delta.push(HashMap::from([('a', 5),('b', 7)]));
    delta.push(HashMap::from([('a', 6)]));
    delta.push(HashMap::from([('a', 2)]));
    delta.push(HashMap::from([('a', 5),('b', 8)]));
    delta.push(HashMap::from([('b', 9)]));
    delta.push(HashMap::new());
    let default = DefaultGraph::new(&delta);

    println!("default edges: {:?}", default.default_edges);
    println!("default components: {:?}", default.components);
    println!("mapping: {:?}", default.mapping);
}
