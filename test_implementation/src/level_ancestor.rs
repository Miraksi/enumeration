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
// TODO what kind of queries do I get?
// we maybe need lifetimes here
// pub only for debugging purposes
pub struct Ladders {
    pub n: usize,
    pub k: usize,
    pub root: usize,
    pub nodes: Vec<Node>,
    pub ladders: Vec<Vec<usize>>,        // ladders are in reverse order // pub for testing
    pub leaf_depth: Vec<(usize, usize)>, // (depth of leaf, index of leaf) // pub for testing
    pub jump_nodes: Vec<usize>,         // stores all jump_nodes (leaves of the macrotree)
    pub jump_points: HashMap<usize, Vec<usize>>,    // stores all the jumppoints for a jump node
                                                // we are allowed to stor this in a HashMap, 
                                                // since inserting will still work in O(n) 
                                                // (n beeing the number of nodes)
    pub micro_table: Vec<Vec<Vec<usize>>>,
}

impl Ladders {
    pub fn new(parent: Vec<usize>, children: Vec<Vec<usize>>, root: usize) -> Self {
        let n = parent.len();
        let k = (log_floor(n as u32)/4) as usize;
        let nodes = compute_node_list(&parent, children);
        let mut new = Self {
            n: n,
            k: k,
            root: root,
            nodes: nodes,
            ladders: Vec::new(),
            leaf_depth: Vec::new(),
            jump_nodes: Vec::new(),
            jump_points: HashMap::new(),
            micro_table: Vec::new(),
        };
        new.compute_ladders(parent);
        new.compute_jump_points();
        new.compute_micro_table();
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
        self.find_jump_nodes(self.root); 
        
        for i in 0..self.jump_nodes.len() {
            self.compute_jumps(self.jump_nodes[i]);
        }
    }

    fn find_jump_nodes(&mut self, root: usize) -> usize{
        let mut decendants = self.nodes[root].children.len();
        let mut child_decendants: usize = 0;
        for i in 0..self.nodes[root].children.len() {
            let child = self.ith_child(root, i);
            let tmp = self.find_jump_nodes(child);
            decendants += tmp;
            child_decendants = max(child_decendants, tmp);
        }
        if decendants >= self.k && child_decendants < self.k {
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
            let current_ladder = self.ladder_of(current);
            match current_ladder.get(ladder_idx + jump_size) {
                Some(x) => current = *x,
                None => {
                    if current_ladder[current_ladder.len()-1] == 0 {
                        current = 0;
                    } else {
                        panic!("index out of bounds in compute_jumps");
                    }
                },  // None should only happen if we would jump to 0
            }; // maybe make this a function
            jumps.push(current);
            jump_size *= 2;
        }
        self.jump_points.insert(base, jumps);
    }

    pub fn graph_to_hash(&self, root: usize) -> u32 {
        let mut hash: u32 = (1 << self.k*2) -1;
        let mut offset: u32 = 0; 
        let mut current = root;
        let mut queue: Vec<usize> = Vec::new();
        for child in self.children_of(current).rev() {
            queue.push(*child);
        }
        while !queue.is_empty() {
            let last = queue[queue.len() - 1];
            if self.parent_of(last) != current {
                current = self.parent_of(current);
            } 
            else {
                hash -= 1 << (offset);
                current = queue.pop().unwrap();
                for child in self.children_of(current).rev() {
                    queue.push(*child);
                }
            }
            offset += 1;
        }
        return hash;
    }

    fn compute_micro_table(&mut self) {
        for hash in 0.. 1 << (2 * self.k) {
            let tmp = brute_level_ancestor(hash_to_graph(self.k, hash as u32));
            self.micro_table.push(tmp);
        }
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

    fn ladder_of(&self, node: usize) -> &Vec<usize> {
        return self.ladders.get(self.nodes[node].ladder).unwrap();
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

pub fn hash_to_graph(k: usize, mut hash: u32) -> Vec<Node> {
    let mut current: usize = 0;
    let mut graph: Vec<Node> = vec![Node::new(0, Vec::new())];
    for _i in 0..2*k {
        if hash % 2 == 0 {
            print!("0");
            let new_idx = graph.len();
            graph[current].children.push(new_idx);
            graph.push(Node::new(current,Vec::new()));
            current = new_idx;
        }
        else {
            print!("1");
            current = graph[current].parent;
        }
        hash = hash / 2;
    }
    println!("");
    return graph;
}

pub fn brute_level_ancestor(graph: Vec<Node>) -> Vec<Vec<usize>> {
    let mut table: Vec<Vec<usize>> = Vec::new();
    for node in 0..graph.len() {
        let mut tmp: Vec<usize> = vec![node];
        let mut current = node;
        while graph[current].parent != current {
            current = graph[current].parent;
            tmp.push(current);
        }
        table.push(tmp);
    }
    return table;
}