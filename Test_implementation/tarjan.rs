struct Tarjan {
    scc: Vec<Vec<usize>>,
    maxdfs: u32,
    unchecked: Vec<bool>,
    stack: Vec<usize>,
    in_stack: Vec<bool>,
    lowlink: Vec<u32>,
    dfs: Vec<u32>,
    edges: Vec<Vec<usize>>,
}

impl Tarjan {
    fn new(n: usize, edges: Vec<Vec<usize>>) -> Self {
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

    fn execute(&mut self) {
        for i in 0..self.unchecked.len() {
            if self.unchecked[i] {
                self.tarjan(i);
            }
        }
    }

    fn tarjan(&mut self, v: usize) {
        self.dfs[v] = self.maxdfs;
        self.lowlink[v] = self.maxdfs;
        self.maxdfs += 1;
        self.stack.push(v);
        self.in_stack[v] = true;
        self.unchecked[v] = false;

        for i in 0..self.edges[v].len() {
            let w = self.edges[v][i];
            if self.unchecked[w] {
                self.tarjan(w);
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
        }
    }
}

fn min(a: u32, b: u32) -> u32 {
    match a < b {
        true => a,
        _ => b,
    } 
}

fn main() {
    let mut tran: Vec<Vec<usize>> = Vec::new();
    tran.push(vec![1,2]);
    tran.push(vec![0,2]);
    tran.push(vec![0,1,3]);
    tran.push(vec![4]);
    tran.push(Vec::new()); 
    let mut test = Tarjan::new(tran.len(), tran);
    test.execute();
    println!("{:?}", test.scc);
}