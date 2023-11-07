mod enumerate;
mod weight;
mod my_math;
pub mod graph_alg;

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
use std::time::{Duration, Instant};

fn main() {
    let mut delta: Vec<Vec<(char, usize)>> = Vec::new();

    delta.push(vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)]);
    delta.push(vec![('a', 2), ('b', 3), ('c', 4), ('d', 0)]);
    delta.push(vec![('a', 3), ('b', 4), ('c', 0), ('d', 1)]);
    delta.push(vec![('a', 4), ('b', 0), ('c', 1), ('d', 2)]);   
    delta.push(vec![('a', 0), ('b', 1), ('c', 2), ('d', 3)]); 

    let start = Instant::now();
    let mut enumerate = Enumerate::new(delta);
    let duration = start.elapsed();
    println!("time needed for initialisation: {:?}", duration);
    enumerate.pmn.show();
    println!("-----------------------------------");
    enumerate.set_n(12);
    let start = Instant::now();
    let mut count = 0;
    enumerate.recurse(' ', 0, 12, 0, &mut count, /*&mut Instant::now()*/);
    let duration = start.elapsed();
    println!("time needed for enumerating {count} words: {:?}", duration);
}

    // delta.push(HashMap::from([('a', 1)]));
    // delta.push(HashMap::from([('a', 2), ('b', 4)]));
    // delta.push(HashMap::from([('a', 3), ('b', 5)]));
    // delta.push(HashMap::from([('a', 3), ('b', 0)]));
    // delta.push(HashMap::from([('a', 6)]));
    // delta.push(HashMap::from([('a', 6)]));
    // delta.push(HashMap::from([]));


    // delta.push(HashMap::from([('a', 1), ('b', 4)]));
    // delta.push(HashMap::from([('a', 2)]));
    // delta.push(HashMap::from([('b', 1), ('a', 3)]));
    // delta.push(HashMap::from([('a', 0)]));
    // delta.push(HashMap::from([('b', 5)]));
    // delta.push(HashMap::from([('b', 6)]));
    // delta.push(HashMap::from([('a', 7),('b', 3)]));
    // delta.push(HashMap::from([('a', 8)]));
    // delta.push(HashMap::from([('b', 8)]));