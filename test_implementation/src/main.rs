mod beq;

use beq::lca::LCA;

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
    let lca = LCA::new(&parent, &children, 0);
    println!("LCA(9,5): {}", lca.get(9,5));
}
