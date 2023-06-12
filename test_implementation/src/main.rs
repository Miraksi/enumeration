mod cartesian;

use cartesian::{cartesian_on_tree};
use std::collections::HashMap;

fn main() {
    let mut children: Vec<Vec<usize>> = Vec::new();

    children.push(vec![1,2]);
    children.push(vec![3,4,5]);
    children.push(vec![6,7]);
    children.push(vec![8]);
    children.push(vec![9,10,11]);
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());

    let parent = vec![0,0,0,1,1,1,2,2,3,4,4,4];
    let mut weights : Vec<Vec<usize>> = vec![Vec::new(); children.len()];
    weights[0] = vec![2,1];
    weights[1] = vec![2,1,3];
    weights[2] = vec![1,4];
    weights[3] = vec![5];
    weights[4] = vec![1,3,3];

    println!("\n\nCartesian Tree\n{:?}",cartesian_on_tree(&parent, &children, &weights, 0));
}
