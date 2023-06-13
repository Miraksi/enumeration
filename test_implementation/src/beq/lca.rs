mod range_min;

use range_min::RMQ;

struct LCA {
    root: usize,
    parent: Vec<usize>,
    children: Vec<Vec<usize>>,
    euler_tour: Vec<u32>,
    idx_map: Vec<usize>,    // stores the index of the Node of the euler tour to the original node
    last_occ: Vec<usize>,   // stores the last occurrence of out inital node in the tour
    rmq: RMQ,
}

impl LCA {
    pub fn new(parent: Vec<usize>, children: Vec<Vec<usize>>, root: usize) -> Self {
        let n = parent.len();
        let mut tmp = Self {
            root: root,
            parent: parent,
            children: children,
            euler_tour: Vec::new(),
            idx_map: Vec::new(),
            last_occ: vec![0; n],
            rmq: RMQ::new(vec![0,1,2,3]),
        };
        tmp.euler_dfs(root, 0);
        tmp.rmq = RMQ::new(tmp.euler_tour.clone());
        return tmp;
    }
    fn euler_dfs(&mut self, root: usize, height: u32) {
        self.last_occ[root] = self.euler_tour.len();
        self.euler_tour.push(height);
        self.idx_map.push(root);
        
        for child in self.children[root].clone().iter() {
            self.euler_dfs(*child, height + 1);
            self.last_occ[root] = self.euler_tour.len();
            self.euler_tour.push(height);
            self.idx_map.push(root);
        }
    }
}

fn main() {
    println!("Hello World!");
}
