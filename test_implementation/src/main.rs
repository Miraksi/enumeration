mod beq;
mod default_graph;
mod path_max_node;
mod level_ancestor;

// use level_ancestor::LevelAncestor;

// fn main() {
//     let parent = vec![0,0,1,2];
//     let children = vec![vec![1], vec![2], vec![3], vec![]];
//     let la = LevelAncestor::new(&parent, &children, 0);
//     println!("LA(3,2) = {}", la.level_ancestor(3,2));
// }


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
    println!("PathMaxNode(4,2) = {}", path_max_node.get(4,2));
}
