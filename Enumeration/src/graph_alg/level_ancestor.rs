extern crate rdxsort;
use rdxsort::*;
use std::slice::Iter;
use crate::my_math::{log_floor, max};


#[derive(Debug)]
/// A data-structure for answering level-ancestor queries.
///
/// # Complexity
/// The preprocessing phase needs O(n) time and queries can be answered in O(1) time.
///
/// # Notes
/// For microtrees one can use better performing preprocessing algorithms. This will not change the overall
/// complexity and therefore isn't done here.
///
/// # Sorces
/// used 'M. A. Bender and M. Farach-Colton. The level ancestor problem simplified.' as reference
pub struct LevelAncestor {
    n: usize,
    k: usize,                        // k = (log n) / 4
    root: usize,
    nodes: Vec<Node>,                // It is important, that the parent of the root, is the root itself
    ladders: Vec<Vec<usize>>,        // ladders are in reverse order
    leaf_depth: Vec<(usize, usize)>, // (depth of leaf, index of leaf)
    jump_nodes: Vec<usize>,         // stores all jump_nodes (leaves of the macrotree)
    jump_points: Vec<Option<Vec<usize>>>,    // stores all the jumppoints for a jump node
    micro_table: Vec<Vec<Vec<usize>>>,
    micro_hashes: Vec<u32>,
    micro_mapping: Vec<Vec<usize>>,     // maps the result of LA on hashes to the actual nodes
}

impl LevelAncestor {
    pub fn new(parent: &Vec<usize>, children: &Vec<Vec<usize>>, root: usize) -> Self {
        let nodes = compute_node_list(parent, children, root);
        let n = nodes.len();
        let k = (log_floor(n as u32)/4) as usize;
        let mut new = Self {
            n: n,
            k: k,
            root: root,
            nodes: nodes,
            ladders: Vec::new(),
            leaf_depth: Vec::new(),
            jump_nodes: Vec::new(),
            jump_points: vec![None; n],
            micro_table: Vec::new(),
            micro_hashes: Vec::new(),
            micro_mapping: Vec::new(),
        };
        new.compute_ladders(parent.clone());
        new.compute_jump_points();
        new.compute_micro_table();
        new.compute_micro_hashes();
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
        self.nodes[p].depth = current_depth;
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
            for child in self.get_children(p) {
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
                top_node = self.get_parent(top_node);
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

    fn find_jump_nodes(&mut self, root: usize) -> (usize, usize){
        let mut descendants = self.nodes[root].children.len();
        let mut child_descendants: usize = 0;
        let mut jump_node: usize = 0;
        for i in 0..self.nodes[root].children.len() {
            let child = self.ith_child(root, i);
            let tmp = self.find_jump_nodes(child);
            descendants += tmp.0;
            child_descendants = max(child_descendants, tmp.0);
            jump_node = tmp.1;
        }
        if descendants >= self.k && child_descendants < self.k {
            self.jump_nodes.push(root);
            jump_node = root;
        }

        self.nodes[root].nearest_jump = jump_node;
        return (descendants, jump_node);
    }

    fn compute_jumps(&mut self, base: usize) {  
        let mut jumps: Vec<usize> = vec![self.get_parent(base)];
        let mut current: usize = self.get_parent(base);
        let mut jump_size: usize = 1;
        while jump_size*2 < self.n {
            let ladder_idx = self.nodes[current].ladder_idx;
            let current_ladder = self.get_ladder(current);
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
        self.jump_points[base] = Some(jumps);
    }

    fn compute_micro_hashes(&mut self) {
        for i in 0..self.jump_nodes.len() {
            let jump_node = self.jump_nodes[i];
            for j in 0..self.nodes[jump_node].children.len() {
                let micro_root = self.nodes[jump_node].children[j];
                let hash = self.graph_to_hash(micro_root, self.micro_hashes.len(), jump_node);
                self.micro_hashes.push(hash);
            }
        }
    }

    // calculates the hash of a graph, 
    // maps the indecies of the hash to the indecies of th original graph,
    // maps nodes to their corresponding micro-tree
    fn graph_to_hash(&mut self, root: usize, micro_tree: usize, nearest_jump: usize) -> u32 {
        let mut hash: u32 = (1 << self.k*2) -1;
        let mut offset: u32 = 0; 
        let mut current = root;
        let mut mapping: Vec<usize> = vec![current];    // to get the mapping afterwards
        self.nodes[current].micro_tree = micro_tree;
        self.nodes[current].micro_idx = 0;
        self.nodes[current].nearest_jump = nearest_jump;

        let mut queue: Vec<usize> = Vec::new();
        for child in self.get_children(current).rev() {
            queue.push(*child);
        }
        while !queue.is_empty() {
            let last = queue[queue.len() - 1];
            if self.get_parent(last) != current {
                current = self.get_parent(current);
            } 
            else {
                hash -= 1 << (offset);
                current = queue.pop().unwrap();
                self.nodes[current].micro_idx = mapping.len();
                mapping.push(current);
                self.nodes[current].micro_tree = micro_tree;
                for child in self.get_children(current).rev() {
                    queue.push(*child);
                }
            }
            offset += 1;
        }
        self.micro_mapping.push(mapping);
        self.nodes[current].nearest_jump = nearest_jump;
        return hash;
    }

    fn compute_micro_table(&mut self) {
        for hash in 0.. 1 << (2 * self.k) {
            let tmp = brute_level_ancestor(hash_to_graph(self.k, hash as u32));
            self.micro_table.push(tmp);
        }
    }

    pub fn level_ancestor(&self, p: usize, l: usize) -> usize {
        let nearest_jump = self.nodes[p].nearest_jump;
        let d: i64 = self.get_depth(nearest_jump) as i64 - self.get_depth(p) as i64;
        if d >= 0 {
            return self.macro_level_ancestor(p, l);
        }
        else if d + l as i64 >= 0 {
            return self.macro_level_ancestor(nearest_jump, (d+l as i64) as usize);
        }
        else {
            return self.micro_level_ancestor(p, l);
        }
    }

    fn macro_level_ancestor(&self, p: usize, l: usize) -> usize {
        if l == 0 {
            return p;
        }
        let node = &self.nodes[p];
        let mut d: i64 = self.get_depth(node.nearest_jump) as i64 - node.depth as i64;
        let jump: usize = log_floor(d as u32 + l as u32) as usize;
        match &self.jump_points[node.nearest_jump] {
            Some(list) => {
                let jumped_to = list[jump];
                d = self.nodes[jumped_to].depth as i64 - node.depth as i64;
                let ladder = self.get_ladder(jumped_to);
                let ladder_idx = self.get_ladder_idx(jumped_to);
                return ladder[(l as i64 + ladder_idx as i64 + d) as usize];
            }
            None => panic!("jump node not found"),
        };
    }

    fn micro_level_ancestor(&self, p: usize, l: usize) -> usize {
        let tree = self.nodes[p].micro_tree;
        let idx = self.nodes[p].micro_idx;
        return self.micro_mapping[tree][self.micro_table[tree][idx][l]];
    }

    #[inline]
    fn ith_child(&self, node: usize, child_idx: usize) -> usize { // maybe add Result-Type
        return self.nodes[node].children[child_idx];
    }

    #[inline]
    fn get_parent(&self, node: usize) -> usize {
        return self.nodes[node].parent;
    }

    #[inline]
    fn get_children(&self, node: usize) -> Iter<usize> {
        return self.nodes[node].children.iter();
    }

    #[inline]
    fn get_ladder(&self, node: usize) -> &Vec<usize> {
        return self.ladders.get(self.nodes[node].ladder).unwrap();
    } 

    #[inline]
    fn get_ladder_idx(&self, node: usize) -> usize {
        return self.nodes[node].ladder_idx;
    }
    
    #[inline]
    fn get_depth(&self, node: usize) -> usize {
        return self.nodes[node].depth;
    }
}

fn compute_node_list(parent: &Vec<usize>, children: &Vec<Vec<usize>>, root: usize) -> Vec<Node> {   
    let mut list: Vec<Node> = Vec::new();
    for i in 0..parent.len() {
        let node = Node::new(parent[i], children[i].clone());
        list.push(node);
    }
    list[root].parent = root;   //Sets the parent of the root to always be the root
    if list.len() < 16 {
        tree_padding(&mut list, root);
    }
    return list;
}

#[inline]
fn tree_padding(list: &mut Vec<Node>, root: usize) {
    while list.len() < 16 {
        list.push(Node::new(root, Vec::new()));
    }
}


fn hash_to_graph(k: usize, mut hash: u32) -> Vec<Node> {
    let mut current: usize = 0;
    let mut graph: Vec<Node> = vec![Node::new(0, Vec::new())];
    for _i in 0..2*k {
        if hash % 2 == 0 {
            // print!("0");
            let new_idx = graph.len();
            graph[current].children.push(new_idx);
            graph.push(Node::new(current,Vec::new()));
            current = new_idx;
        }
        else {
            // print!("1");
            current = graph[current].parent;
        }
        hash = hash / 2;
    }
    return graph;
}

fn brute_level_ancestor(graph: Vec<Node>) -> Vec<Vec<usize>> {
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

#[derive(Debug)]
struct Node {
    parent: usize,
    children: Vec<usize>,
    depth: usize,
    ladder: usize,
    ladder_idx: usize,
    nearest_jump: usize,  //points to the closest jump node
    micro_tree: usize,
    micro_idx: usize,
}
impl Node {
    fn new(parent: usize, children: Vec<usize>) -> Self {
        Self{
            parent: parent,
            children: children,
            depth: 0,
            ladder: 0,
            ladder_idx: 0,
            nearest_jump: 0,
            micro_tree: 0,
            micro_idx: 0,
        }
    }
}
