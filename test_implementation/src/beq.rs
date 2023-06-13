pub mod cartesian;
mod lca;

use cartesian::{Node, cartesian_on_tree};


struct Bottleneck {
    c_tree: Vec<cartesian::Node>,
}

impl Bottleneck {
    pub fn new(parent: &Vec<usize>, children: &Vec<Vec<usize>>, weights: &Vec<Vec<usize>>, root: usize) -> Self {
        Self{
            c_tree: cartesian_on_tree(parent, children, weights, root),
        }
    }
}