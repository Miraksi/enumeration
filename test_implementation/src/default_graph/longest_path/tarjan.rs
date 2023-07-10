/// A data-structure for executing tarjan's strongly connected components algorithm.
///
/// # Complexity
/// Runs in O(|V| + |E|) time
///
/// # Sources
/// used https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm as reference
pub struct Tarjan<'a> {
    scc: Vec<Vec<usize>>,
    maxdfs: u32,
    unchecked: Vec<bool>,
    stack: Vec<usize>,
    in_stack: Vec<bool>,
    lowlink: Vec<u32>,
    dfs: Vec<u32>,
    edges: &'a Vec<Vec<usize>>,
}

impl<'a> Tarjan<'a> {
    pub fn new(edges: &'a Vec<Vec<usize>>) -> Self {
        let n = edges.len();    // assumes that every node has an entry in edges
        Self{
            scc: Vec::new(),
            maxdfs: 0,
            unchecked: vec![true;n],
            stack: Vec::new(),
            in_stack: vec![false;n],
            lowlink: vec![0;n],
            dfs: vec![0;n],
            edges: edges
        }
    }

    pub fn tarjan(&mut self) -> Vec<Vec<usize>> {
        for i in 0..self.unchecked.len() {
            if self.unchecked[i] {
                self.execute(i);
            }
        }
        return self.scc.clone();
    }

    fn execute(&mut self, v: usize) {
        self.dfs[v] = self.maxdfs;
        self.lowlink[v] = self.maxdfs;
        self.maxdfs += 1;
        self.stack.push(v);
        self.in_stack[v] = true;
        self.unchecked[v] = false;

        for i in 0..self.edges[v].len() {
            let w = self.edges[v][i];
            if self.unchecked[w] {
                self.execute(w);
                self.lowlink[v] = min(self.lowlink[v], self.lowlink[w]);
            }
            else if self.in_stack[w] {
                self.lowlink[v] = min(self.lowlink[v], self.dfs[w]);
            }
        }

        if self.lowlink[v] == self.dfs[v] {
            let mut component: Vec<usize> = Vec::new(); 
            let mut w = self.stack.pop().unwrap();
            self.in_stack[w] = false;
            while w != v {
                component.push(w);
                w = self.stack.pop().unwrap();
                self.in_stack[w] = false;
            }
            component.push(w);
            if component.len() > 1 {
                self.scc.push(component);
            }
            else if self.has_loop(v) {
                self.scc.push(component);
            }
        }
    }

    fn has_loop(&self, v: usize) -> bool {
        for w in self.edges[v].iter() {
            if *w == v {
                return true;
            }
        }
        return false;
    }
}

fn min(a: u32, b: u32) -> u32 {
    match a < b {
        true => a,
        _ => b,
    } 
}