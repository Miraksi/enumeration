/// A data-structure for computing a topological sorting of a given graph
///
/// # Complexity 
/// O(|E| + |V|)
///
/// # Notes 
/// graph is given by its edges, which leads to problems with single vertecies
pub struct Toposort<'a> {
    edges: &'a Vec<Vec<usize>>,
    used: Vec<bool>,
    toposort: Vec<usize>,
}

impl <'a> Toposort<'a> {
    pub fn new(edges: &'a Vec<Vec<usize>>) -> Self {
        let n = edges.len();
        Self {
            edges: edges,
            used: vec![false;n],
            toposort: Vec::new(),
        }
    }
    pub fn reverse_toposort(&mut self) -> Vec<usize> {    
        let n = self.edges.len();
        for i in 0..n {
            if !self.used[i] {
                self.dfs_toposort(i);
            }
        }
        return self.toposort.clone();
    }
    fn dfs_toposort(&mut self, v: usize) {
        self.used[v] = true;
        for u in self.edges[v].iter() {
            if !self.used[*u] {
                self.dfs_toposort(*u);
            }
        }
        self.toposort.push(v);
    }
    
}