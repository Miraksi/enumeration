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
}



fn main() {}