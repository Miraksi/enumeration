pub mod cartesian;
pub mod lca;

use cartesian::{Node, cartesian_on_tree, cartesian_to_tree};
use lca::LCA;


pub struct Bottleneck {
    pub c_tree: Vec<Node>,
    pub last_occ: Vec<usize>,
    pub lca: LCA,
}

impl Bottleneck {
    pub fn new(parent: Vec<usize>, children: Vec<Vec<usize>>, weights: Vec<Vec<i64>>, root: usize) -> Self {
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

// fn main() {
//     let mut children: Vec<Vec<usize>> = Vec::new();

//     children.push(vec![1,2]);
//     children.push(vec![3,4,5]);
//     children.push(vec![6,7]);
//     children.push(vec![8]);
//     children.push(vec![9,10,11]);
//     children.push(Vec::new());
//     children.push(Vec::new());
//     children.push(Vec::new());
//     children.push(Vec::new());
//     children.push(Vec::new());
//     children.push(Vec::new());
//     children.push(Vec::new());

//     let parent = vec![0,0,0,1,1,1,2,2,3,4,4,4];
//     let mut weights: Vec<Vec<usize>> = vec![Vec::new(); children.len()];
//     weights[0] = vec![8,1];
//     weights[1] = vec![7,1,9];
//     weights[2] = vec![1,4];
//     weights[3] = vec![5];
//     weights[4] = vec![1,6,3];

//     let b = Bottleneck::new(&parent, &children, &weights, 0);
//     println!("{:?}", b.c_tree);
//     println!("{:?}", b.last_occ);
//     println!("\nbeq(3,5) = {}", b.get(3,5));
// }