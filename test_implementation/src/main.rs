mod cartesian;

use cartesian::{cartesian_on_tree};
use std::collections::HashMap;

fn main() {
    let mut children: Vec<Vec<usize>> = Vec::new();

    children.push(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());

    let parent = vec![0;children.len()];
    let mut weights : Vec<Vec<usize>> = vec![Vec::new(); children.len()];
    weights[0] = vec![1;children[0].len()];
    weights[0][2] = 4;
    weights[0][6] = 2;

    cartesian_on_tree(&parent, &children, &weights, 0);
}
