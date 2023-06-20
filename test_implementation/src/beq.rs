pub mod cartesian;
pub mod lca;

use cartesian::{Node, cartesian_on_tree};


pub struct Bottleneck {
    pub c_tree: Vec<cartesian::Node>,
    pub last_occ: Vec<usize>,
}

impl Bottleneck {
    pub fn new(parent: &Vec<usize>, children: &Vec<Vec<usize>>, weights: &Vec<Vec<usize>>, root: usize) -> Self {
        let (c_tree, last_occ) = cartesian_on_tree(parent, children, weights, root);
        Self{
            c_tree: c_tree,
            last_occ: last_occ,
        }
    }
}