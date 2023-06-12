extern crate rdxsort;
mod connectivity;

use rdxsort::*;
use connectivity::{Connectivity, Component};

#[derive(Clone,Debug)]
pub struct Node {
    parent: usize,
    left: Option<usize>,
    right: Option<usize>,
    weight: usize,
}
impl Node {
    pub fn new(parent: usize, left: Option<usize>, right: Option<usize>, weight: usize) -> Self {
        Node{
            parent: parent,
            left: left,
            right: right,
            weight: weight,
        }
    }
}

pub fn cartesian_on_tree(parent: &Vec<usize>, children: &Vec<Vec<usize>>, weights: &Vec<Vec<usize>>, root: usize) {
    let mut con = Connectivity::new(parent, children, root);
    let mut edge_lst: Vec<(usize,(usize, usize))> = Vec::new();
    let mut c_tree: Vec<Node> = Vec::new();
    

    for i in 0..children.len() {
        for j in 0..children[i].len() {
            edge_lst.push((weights[i][j],(i,children[i][j])));
        }
    }
    edge_lst.rdxsort();
    println!("Weighted edges: {:?}", edge_lst);

    for (w,(u,v)) in edge_lst.iter() {
        let len = c_tree.len();
        let mut tmp = Node::new(len, None, None, *w);
        let comp_idx = con.get_comp_idx(*u);
        // if let Some(x) = con.comp_list[*u].parent {
        //     tmp.parent = x;
        //     match comp_list[*u].side {
        //         Some(Side::Left) => c_tree[x].left = Some(len),
        //         Some(Side::Right) => c_tree[x].right = Some(len),
        //         _ => panic!("no side found"),
        //     };
        // }
        c_tree.push(tmp);

        con.delete(*u, *v);
    }
}