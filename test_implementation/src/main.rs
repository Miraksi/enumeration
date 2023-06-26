mod beq;
mod default_graph;
mod path_max_node;
mod level_ancestor;




use std::collections::HashMap;
use path_max_node::PathMaxNode;

fn main() {
    let mut delta: Vec<HashMap<char, usize>> = Vec::new();

    delta.push(HashMap::from([('a', 1), ('b', 4)]));
    delta.push(HashMap::from([('a', 2)]));
    delta.push(HashMap::from([('b', 1), ('1', 3)]));
    delta.push(HashMap::from([('a', 3)]));
    delta.push(HashMap::from([('b', 5)]));
    delta.push(HashMap::from([('b', 6)]));
    delta.push(HashMap::from([('b', 3)]));

    let path_max_node = PathMaxNode::new(&delta);
    path_max_node.show();
}
