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

    delta.push(vec![('a', 1), ('b', 4)]);
    delta.push(vec![('a', 2)]);
    delta.push(vec![('b', 1), ('a', 3)]);
    delta.push(vec![('a', 0)]);
    delta.push(vec![('b', 5),('a', 9)]);
    delta.push(vec![('b', 6),('a', 10)]);
    delta.push(vec![('a', 7),('b', 3)]);
    delta.push(vec![('a', 8)]);
    delta.push(vec![('b', 8)]);
    delta.push(vec![('a', 11),('b', 12)]);
    delta.push(vec![('a', 13),('b', 14)]);
    delta.push(vec![('a', 15),('b', 16)]);
    delta.push(vec![('a', 17),('b', 18)]);
    delta.push(vec![('a', 19),('b', 20)]);
    delta.push(vec![('a', 21),('b', 22)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![]);

    let start = Instant::now();
    let n = 30;
    let mut enumerate = Enumerate::new(delta, n);
    let duration = start.elapsed();
    println!("time needed for initialisation: {:?}", duration);
    enumerate.pmn.show();
    println!("-----------------------------------");
    let start = Instant::now();
    let mut count = 0;
    let mut stack_s = Vec::new();
    enumerate.recurse(' ', 0, n, 0, &mut stack_s, &mut count, /*&mut Instant::now()*/);
    let duration = start.elapsed();
    println!("time needed for enumerating {count} words: {:?}", duration);
}




