pub mod cartesian;
pub mod lca;

use cartesian::{Node, cartesian_on_tree, cartesian_to_tree};
use lca::LCA;
use crate::weight::Weight;

/// A data-structure for retrieving the minimal edge-value along a path from u to v through a tree.
///
/// # Complexity
/// The preprocessing phase runs in O(n) time and queries can be answered in O(1) time.
///
/// # Sources
/// used 'E. D. Demaine, G. M. Landau, and O. Weimann. On cartesian trees and range minimum queries' as reference
pub struct Bottleneck {
    pub c_tree: Vec<Node>,
    pub last_occ: Vec<usize>,
    pub lca: LCA,
}

impl Bottleneck {
    pub fn new(parent: Vec<usize>, children: Vec<Vec<usize>>, weights: Vec<Vec<Weight>>, root: usize) -> Self {
        let (c_tree, last_occ) = cartesian_on_tree(&parent, &children, &weights, root);
        let (c_parent, c_children) = cartesian_to_tree(&c_tree);    // this returns parent and children of the cartesian tree while keeping the indices
        Self{
            c_tree: c_tree,
            last_occ: last_occ,
            lca: LCA::new(&c_parent, &c_children, 0),
        }
    }

    pub fn get(&self, a: usize, b: usize) -> &Node {
        let ancestor = self.lca.get(self.last_occ[a], self.last_occ[b]);
        return &self.c_tree[ancestor];
    }
}