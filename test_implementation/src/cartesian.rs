extern crate rdxsort;
mod connectivity;

use rdxsort::*;
use connectivity::{Connectivity, Component};

enum Side {
    Left,
    Right,
}

pub fn cartesian_on_tree(parent: &Vec<usize>, children: &Vec<Vec<usize>>, weights: &Vec<Vec<usize>>, root: usize) {
    let mut con = Connectivity::new(parent, children, root);
    let mut edge_lst: Vec<(usize,(usize, usize))> = Vec::new();

    for i in 0..children.len() {
        for j in 0..children[i].len() {
            edge_lst.push((weights[i][j],(i,children[i][j])));
        }
    }
    edge_lst.rdxsort();
    println!("Weighted edges: {:?}", edge_lst);
}