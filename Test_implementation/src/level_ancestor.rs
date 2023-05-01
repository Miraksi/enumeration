extern crate rdxsort;
use rdxsort::*;

fn log_floor(x: u32) -> u32 {   // TODO outsource this code into a module
    return u32::BITS - x.leading_zeros() - 1;
}
// we maybe need lifetimes here
pub struct Ladders {
    root: usize,
    parent: Vec<usize>,
    children: Vec<Vec<usize>>,
    pub ladders: Vec<Vec<usize>>,        // pub for testing
    pub leaf_depth: Vec<(usize, usize)>, // (depth of leaf, index of leaf) // pub for testing
    ladder_of_node: Vec<usize>,     // stores the index of the ladder on which a node lies
    jump_nodes: Vec<usize>,         // stores all jump_nodes (leaves of the macrotree)
}

impl Ladders {
    pub fn new(parent: Vec<usize>, children: Vec<Vec<usize>>, root: usize) -> Self {
        let n = parent.len();
        let mut new = Self {
            root: root,
            parent: parent,
            children: children,
            ladders: Vec::new(),
            leaf_depth: Vec::new(),
            ladder_of_node: vec![0;n],
            jump_nodes: Vec::new(),
        };
        new.compute_ladders();
        new.find_jump_nodes(root, (log_floor(n as u32)/4) as usize);
        return new;
    }

    fn compute_ladders(&mut self) {
        self.dfs_depth(self.root, 0);
        self.leaf_depth.rdxsort();
        self.leaf_depth.reverse();
        let original_parent = self.parent.clone();

        for i in 0..self.leaf_depth.len() {
            let (_h, p) = self.leaf_depth[i];
            self.cut(p);
        }
        self.parent = original_parent;
        self.path_to_ladders();
    }

    fn dfs_depth(&mut self, p: usize, current_depth: usize) {
        if self.children[p].len() == 0 {
            self.leaf_depth.push((current_depth, p));
            return;
        }
        for i in 0..self.children[p].len() {
            let child = self.children[p][i];
            self.dfs_depth(child, current_depth + 1);
        }
    }

    fn cut(&mut self, mut p: usize) {
        let mut long_path: Vec<usize> = vec![p];
        while self.parent[p] != p {
            p = self.parent[p];
            long_path.push(p);
            self.ladder_of_node[p] = self.ladders.len();
            for child in self.children[p].iter() {
                if *child != p {
                    self.parent[*child] = *child;
                }    
            }
        }
        self.ladders.push(long_path);  // this will add a reverse long_path 
    }

    fn path_to_ladders(&mut self) {
        for  i in 0..self.ladders.len() {
            let h = self.ladders[i].len();
            let mut top_node = self.ladders[i][h-1];
            for _i in 0..h {
                top_node = self.parent[top_node];
                self.ladders[i].push(top_node);
            }
        }
    }

    fn find_jump_nodes(&mut self, root: usize, boundary: usize) -> usize{
        let mut decendants = self.children[root].len();
        let mut child_decendants: usize = 0;
        for i in 0..self.children[root].len() {
            let child = self.children[root][i];
            let tmp = self.find_jump_nodes(child, boundary);
            decendants += tmp;
            child_decendants = max(child_decendants, tmp);
        }
        if decendants >= boundary && child_decendants < boundary {
            self.jump_nodes.push(root);
        }
        return decendants;
    }
}

fn max(a: usize, b: usize) -> usize { // TODO outsource
    match a < b {
        true => b,
        false => a,
    }
}


fn main() {}