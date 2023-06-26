extern crate rdxsort;
mod connectivity;

use rdxsort::*;
use connectivity::{Connectivity, Component, Side};

#[derive(Clone,Debug)]
pub struct Node {
    parent: usize,
    left: Option<usize>,
    right: Option<usize>,
    pub weight: i64,  //TODO Check if weight is necessary
    pub edge: (usize, usize),
}
impl Node {
    pub fn new(parent: usize, left: Option<usize>, right: Option<usize>, weight: i64, edge: (usize, usize)) -> Self {
        Node{
            parent: parent,
            left: left,
            right: right,
            weight: weight,
            edge: edge,
        }
    }
}

//TODO refactor method to be smaller
pub fn cartesian_on_tree(parent: &Vec<usize>, children: &Vec<Vec<usize>>, weights: &Vec<Vec<i64>>, root: usize) -> (Vec<Node>, Vec<usize>) {
    let mut con = Connectivity::new(parent, children, root);
    let mut edge_lst: Vec<(usize,(usize, usize))> = Vec::new();
    let mut c_tree: Vec<Node> = Vec::new();
    let mut side_list: Vec<Option<Side>> = vec![None; parent.len()];
    let mut last_occ: Vec<usize> = vec![0; parent.len()];
    
    let edge_lst = sorted_edge_list(children, weights);

    for (weight,(u,v)) in edge_lst.iter() {
        let len = c_tree.len();
        let mut tmp = Node::new(len, None, None, *weight, (*u, *v));
        let comp_idx = con.get_comp_idx(*u);
        if let Some(x) = con.comp_list[comp_idx].parent {
            tmp.parent = x;
            match con.comp_list[comp_idx].side {
                Some(x) => side_list[len] = Some(x),
                _ => panic!("no side found"),
            };
        }
        last_occ[*u] = max(last_occ[*u], len);
        last_occ[*v] = max(last_occ[*v], len);
        c_tree.push(tmp);

        // println!("deleting ({}, {})", *u, *v);
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
    return (c_tree, last_occ);
}

fn sorted_edge_list(children: &Vec<Vec<usize>>, weights: &Vec<Vec<i64>>) -> Vec<(i64, (usize, usize))>{
    let mut edge_lst = Vec::new();
    for i in 0..children.len() {
        for j in 0..children[i].len() {
            edge_lst.push((weights[i][j],(i,children[i][j])));
        }
    }
    edge_lst.rdxsort();
    return edge_lst;
}

fn max(a: usize, b: usize) -> usize {
    if a < b {
        return b;
    }
    return a; 
}

pub fn cartesian_to_tree(c_tree: &Vec<Node>) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut parent: Vec<usize> = vec![0; c_tree.len()];
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); c_tree.len()];

    for i in 0..c_tree.len() {
        parent[i] = c_tree[i].parent;
        if let Some(x) = c_tree[i].left {
            children[i].push(x);
        }
        if let Some(x) = c_tree[i].right {
            children[i].push(x);
        }
    }
    return (parent, children);
}

// taken from https://cp-algorithms.com/graph/rmq_linear.html#construction-of-a-cartesian-tree
// does keep the indicies, so no mapping needed
pub fn cartesian_on_list(list: &Vec<i64>) -> (usize, Vec<usize>, Vec<Vec<usize>>) {
    let mut stack: Vec<usize> = Vec::new();
    let mut parent: Vec<usize> = vec![list.len(); list.len()];
    for i in 0..list.len() {
        let mut last: Option<usize> = None;
        let mut len = stack.len();
        while !stack.is_empty() && list[stack[len - 1]] >= list[i] {
            last = Some(stack.pop().unwrap());
            len -= 1;
        }
        if !stack.is_empty() {
            parent[i] = stack[len - 1];
        }
        if let Some(idx) = last {
            parent[idx] = i;
        }
        stack.push(i);
    }
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); list.len()];
    let mut root = list.len();
    for i in 0..parent.len() {
        if parent[i] == list.len() {
            root = i;
            parent[i] = i;
        }
        children[parent[i]].push(i);
    }
    return (root, parent, children);
}