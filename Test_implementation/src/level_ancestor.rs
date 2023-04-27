extern crate rdxsort;
use rdxsort::*;

struct Tree {
    parent: Vec<usize>,
    children: Vec<Vec<usize>>,
}

// we maybe need lifetimes here
pub struct Ladders {
    root: usize,
    parent: Vec<usize>,
    children: Vec<Vec<usize>>,
    pub ladders: Vec<Vec<usize>>,        // pub for testing
    pub leaf_depth: Vec<(usize, usize)>, // (depth of leaf, index of leaf) // pub for testing
}

impl Ladders {
    pub fn new(parent: Vec<usize>, children: Vec<Vec<usize>>, root: usize) -> Self {
        let mut new = Self {
            root: root,
            parent: parent,
            children: children,
            ladders: Vec::new(),
            leaf_depth: Vec::new(),
        };
        new.compute_ladders();
        return new;
    }

    fn compute_ladders(&mut self) {
        self.dfs_depth(self.root, 0);
        self.leaf_depth.rdxsort();
        self.leaf_depth.reverse();

        for i in 0..self.leaf_depth.len() {
            let (_h, p) = self.leaf_depth[i];
            self.cut(p);
        }
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
        let mut ladder: Vec<usize> = vec![p];
        while self.parent[p] != p {
            p = self.parent[p];
            ladder.push(p);
            for child in self.children[p].iter() {
                if *child != p {
                    self.parent[*child] = *child;
                }    
            }
        }
        ladder.reverse();
        self.ladders.push(ladder);
    }
}



fn main() {}