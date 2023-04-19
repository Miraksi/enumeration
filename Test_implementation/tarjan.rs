use std::collections::{HashSet, HashMap};

struct Tarjan {
    maxdfs: u32,
    unchecked: Vec<bool>,
    stack: Vec<usize>,
    lowlink: Vec<u32>,
    dfs: Vec<u32>,
    edges: Vec<Vec<usize>>,
}

impl Tarjan {
    fn new(n: usize, edges: Vec<Vec<usize>>) -> Self {
        Self{
            maxdfs: 0,
            unchecked: vec![true;n],
            stack: Vec::new(),
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
        self.unchecked[v] = false;

        for i in 0..self.edges[v].len() {
            let w = self.edges[v][i];
            if self.unchecked[w] {
                self.tarjan(w);
                self.lowlink[v] = min(self.lowlink[v], self.lowlink[w]);
            }
            else if self.stack.contains(&w) {
                self.lowlink[v] = min(self.lowlink[v], self.dfs[w]);
            }
        }

        if self.lowlink[v] == self.dfs[v] {
            print!("SZK: ");
            let mut w = self.stack.pop().unwrap();
            while w != v {
                print!("{} ", w);
                w = self.stack.pop().unwrap();
            }
            println!("{}",w);
        }
    }
}

fn min(a: u32, b: u32) -> u32 {
    match a < b {
        true => a,
        _ => b,
    } 
}

fn main() {}