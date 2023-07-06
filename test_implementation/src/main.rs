mod beq;
mod default_graph;
mod enumerate;
mod level_ancestor;
mod weight;
mod my_math;

// use beq::cartesian::connectivity::Connectivity;

// fn main() {
//     let parent = vec![0];
//     let children = vec![vec![]];
//     let con = Connectivity::new(&parent, &children, 0);
// }

// use beq::lca::LCA;

// fn main() {
//     let parent = vec![];
//     let children = vec![];
//     let tmp = LCA::new(&parent, &children, 0);
// }

use std::collections::HashMap;
use enumerate::Enumerate;

fn main() {
    let mut delta: Vec<HashMap<char, usize>> = Vec::new();

    delta.push(HashMap::from([('a', 1), ('b', 4)]));
    delta.push(HashMap::from([('a', 2)]));
    delta.push(HashMap::from([('b', 1), ('a', 3)]));
    delta.push(HashMap::from([('a', 0)]));
    delta.push(HashMap::from([('b', 5)]));
    delta.push(HashMap::from([('b', 6)]));
    delta.push(HashMap::from([('a', 7),('b', 3)]));
    delta.push(HashMap::from([('a', 8)]));
    delta.push(HashMap::from([('b', 8)]));

    let mut enumerate = Enumerate::new(delta);
    println!("initialized");
    enumerate.pmn.show();
    println!("-----------------------------------");
    enumerate.set_n(9);
    enumerate.recurse(' ', 0, 9, 0);
}

// delta.push(HashMap::from([('a', 1)]));
// delta.push(HashMap::from([('a', 2), ('b', 4)]));
// delta.push(HashMap::from([('a', 3), ('b', 5)]));
// delta.push(HashMap::from([('a', 3)]));
// delta.push(HashMap::from([('a', 6)]));
// delta.push(HashMap::from([('a', 6)]));
// delta.push(HashMap::from([('a', 6)]));


