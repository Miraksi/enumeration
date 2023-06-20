mod beq;
mod default_graph;

use beq::Bottleneck;


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
    let mut weights: Vec<Vec<usize>> = vec![Vec::new(); children.len()];
    weights[0] = vec![8,1];
    weights[1] = vec![7,1,9];
    weights[2] = vec![1,4];
    weights[3] = vec![5];
    weights[4] = vec![1,6,3];

    let b = Bottleneck::new(&parent, &children, &weights, 0);
    println!("{:?}", b.c_tree);
    println!("{:?}", b.last_occ);
    println!("\nbeq(3,5) = {}", b.get(3,5));
}
