extern crate rdxsort;
mod connectivity;

use rdxsort::*;
use connectivity::{Connectivity, Component, Side};

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

pub fn cartesian_on_tree(parent: &Vec<usize>, children: &Vec<Vec<usize>>, weights: &Vec<Vec<usize>>, root: usize) -> Vec<Node>{
    let mut con = Connectivity::new(parent, children, root);
    let mut edge_lst: Vec<(usize,(usize, usize))> = Vec::new();
    let mut c_tree: Vec<Node> = Vec::new();
    let mut side_list: Vec<Option<Side>> = vec![None; parent.len()];
    

    for i in 0..children.len() {
        for j in 0..children[i].len() {
            edge_lst.push((weights[i][j],(i,children[i][j])));
        }
    }
    edge_lst.rdxsort();

    for (weight,(u,v)) in edge_lst.iter() {
        let len = c_tree.len();
        let mut tmp = Node::new(len, None, None, *weight);
        let comp_idx = con.get_comp_idx(*u);
        if let Some(x) = con.comp_list[comp_idx].parent {
            tmp.parent = x;
            match con.comp_list[comp_idx].side {
                Some(x) => side_list[len] = Some(x),
                _ => panic!("no side found"),
            };
        }
        c_tree.push(tmp);

        println!("deleting ({}, {})", *u, *v);
        con.delete(*u, *v);

        let u_idx = con.get_comp_idx(*u);
        con.comp_list[u_idx].parent = Some(len);
        con.comp_list[u_idx].side = Some(Side::Left);
        let v_idx = con.get_comp_idx(*v);
        con.comp_list[v_idx].parent = Some(len);
        con.comp_list[v_idx].side = Some(Side::Right);
    }

    for i in 0..c_tree.len() {
        let p = c_tree[i].parent;
        if p == i {
            continue;
        }
        match side_list[i] {
            Some(Side::Left) => c_tree[p].left = Some(i),
            Some(Side::Right) => c_tree[p].right = Some(i),
            None => panic!("node has no side"),
        };
    }
    return c_tree;
}