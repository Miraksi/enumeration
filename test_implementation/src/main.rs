mod level_ancestor;

use level_ancestor::{Ladders,hash_to_graph,brute_level_ancestor};

fn main() {
    let parent: Vec<usize> = vec![0,0,1,0,3,3,2,6,5,4,4,8,9,10,10,7,7];
    let children: Vec<Vec<usize>> = vec![vec![1,3], vec![2], vec![6], vec![4,5], vec![9,10], vec![8], vec![7], vec![15,16], vec![11], vec![12], vec![13,14], Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let lad = Ladders::new(parent, children, 0);
    println!("k: {}", lad.k);
    println!("{:?}", lad.ladders);
    println!("jump_nodes {:?}", lad.jump_nodes);
    println!("macro_level_ancestor(5,2) = {:?}", lad.micro_level_ancestor(16,0));
}
