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
    ladders: Vec<Vec<usize>>,
    pub leaf_height: Vec<(usize, usize)>, // (height of leaf, index of leaf)
}

impl Ladders {
    pub fn new(parent: Vec<usize>, children: Vec<Vec<usize>>, root: usize) -> Self {
        let mut new = Self {
            root: root,
            parent: parent,
            children: children,
            ladders: Vec::new(),
            leaf_height: Vec::new(),
        };
        new.dfs_height(root, 0);
        new.leaf_height.rdxsort();
        return new;
    }
    fn dfs_height(&mut self, p: usize, current_height: usize) {
        if self.children[p].len() == 0 {
            self.leaf_height.push((current_height, p));
            return;
        }
        for i in 0..self.children[p].len() {
            let child = self.children[p][i];
            self.dfs_height(child, current_height + 1);
        }
    }
}



fn main() {}