extern crate rdxsort;
use rdxsort::*;
use std::collections::HashMap;
use std::slice::Iter;

fn log_floor(x: u32) -> u32 {   // TODO outsource this code into a module
    return u32::BITS - x.leading_zeros() - 1;
}
#[derive(Debug)]
pub struct Node {
    parent: usize,
    children: Vec<usize>,
    ladder: usize,
    ladder_idx: usize,
}
impl Node {
    fn new(parent: usize, children: Vec<usize>) -> Self {
        Self{
            parent: parent,
            children: children,
            ladder: 0,
            ladder_idx: 0,
        }
    }
}

// we maybe need lifetimes here
pub struct Ladders {
    n: usize,
    root: usize,
    nodes: Vec<Node>,
    pub ladders: Vec<Vec<usize>>,        // ladders are in reverse order // pub for testing
    pub leaf_depth: Vec<(usize, usize)>, // (depth of leaf, index of leaf) // pub for testing
    jump_nodes: Vec<usize>,         // stores all jump_nodes (leaves of the macrotree)
    jump_points: HashMap<usize, Vec<usize>>,    // stores all the jumppoints for a jump node
                                                // we are allowed to stor this in a HashMap, 
                                                // since inserting will still work in O(n) 
                                                // (n beeing the number of nodes)
}

impl Ladders {
    pub fn new(parent: Vec<usize>, children: Vec<Vec<usize>>, root: usize) -> Self {
        let n = parent.len();
        let nodes = compute_node_list(&parent, children);
        let mut new = Self {
            n: n,
            root: root,
            nodes: nodes,
            ladders: Vec::new(),
            leaf_depth: Vec::new(),
            jump_nodes: Vec::new(),
            jump_points: HashMap::new(),
        };
        new.compute_ladders(parent);
        new.compute_jump_points();
        return new;
    }

    fn compute_ladders(&mut self, mut parent: Vec<usize>) {
        self.dfs_depth(self.root, 0);
        self.leaf_depth.rdxsort();
        self.leaf_depth.reverse();
        for i in 0..self.leaf_depth.len() {
            let (_h, p) = self.leaf_depth[i];
            self.cut(p, &mut parent);
        }
        self.path_to_ladders();
    }

    fn dfs_depth(&mut self, p: usize, current_depth: usize) {
        if self.nodes[p].children.len() == 0 {
            self.leaf_depth.push((current_depth, p));
            return;
        }
        for i in 0..self.nodes[p].children.len() {
            let child = self.ith_child(p,i);
            self.dfs_depth(child, current_depth + 1);
        }
    }

    fn cut(&mut self, mut p: usize, parent: &mut Vec<usize>) {
        let mut long_path: Vec<usize> = vec![p];
        while parent[p] != p {
            p = parent[p];
            long_path.push(p);
            self.nodes[p].ladder = self.ladders.len();
            for child in self.children_of(p) {
                if *child != p {
                    parent[*child] = *child;
                }    
            }
        }
        self.ladders.push(long_path);  // this will add a reverse long_path (so starting from a leaf)
    }

    fn path_to_ladders(&mut self) {    // ladders maybe have to be reversed, but change idx then
        for  i in 0..self.ladders.len() {
            let h = self.ladders[i].len();
            let mut top_node = self.ladders[i][h-1];
            for j in 0..h {
                top_node = self.parent_of(top_node);
                self.ladders[i].push(top_node);
                self.nodes[self.ladders[i][j]].ladder_idx = j; // set ladder index for noded on the ladder
            }
        }
    }

    fn compute_jump_points(&mut self) {
        let bound = (log_floor(self.n as u32)/4) as usize;
        self.find_jump_nodes(self.root, bound); 
        
        for i in 0..self.jump_nodes.len() {
            self.compute_jumps(self.jump_nodes[i]);
        }
    }

    fn find_jump_nodes(&mut self, root: usize, boundary: usize) -> usize{
        let mut decendants = self.nodes[root].children.len();
        let mut child_decendants: usize = 0;
        for i in 0..self.nodes[root].children.len() {
            let child = self.ith_child(root, i);
            let tmp = self.find_jump_nodes(child, boundary);
            decendants += tmp;
            child_decendants = max(child_decendants, tmp);
        }
        if decendants >= boundary && child_decendants < boundary {
            self.jump_nodes.push(root);
        }
        return decendants;
    }

    // TODO test
    fn compute_jumps(&mut self, base: usize) {  // Maybe this should return a Vector
        let mut jumps: Vec<usize> = vec![self.parent_of(base)];
        let mut current: usize = self.parent_of(base);
        let mut jump_size: usize = 1;
        while jump_size*2 < self.n {
            let ladder_idx = self.nodes[current].ladder_idx;
            current = self.ladders[self.ladder_of(current)][ladder_idx + jump_size]; // maybe make this a function
            jumps.push(current);
            jump_size *= 2;
        }
        self.jump_points.insert(base, jumps);
    }

    // TODO check if #[inline] should be added
    fn ith_child(&self, node: usize, child_idx: usize) -> usize { // maybe add Result-Type
        return self.nodes[node].children[child_idx];
    }

    fn parent_of(&self, node: usize) -> usize {
        return self.nodes[node].parent;
    }

    fn children_of(&self, node: usize) -> Iter<usize> {
        return self.nodes[node].children.iter();
    }

    fn ladder_of(&self, node: usize) -> usize {
        return self.nodes[node].ladder;
    } 
}

pub fn compute_node_list(parent: &Vec<usize>, children: Vec<Vec<usize>>) -> Vec<Node> {   
    let mut list: Vec<Node> = Vec::new();
    for i in 0..parent.len() {
        let node = Node::new(parent[i], children[i].clone());
        list.push(node);
    }
    return list;
}

fn max(a: usize, b: usize) -> usize { // TODO outsource
    match a < b {
        true => b,
        false => a,
    }
}