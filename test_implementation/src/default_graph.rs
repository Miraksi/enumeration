mod longest_path;

use std::collections::HashMap;
use longest_path::compute_longest_pairs;

#[derive(Debug)]
enum CompType {
    Independent,
    Connected,
    Cycle,
}

#[derive(Debug)]
pub struct DefaultComp {
    comp_typ: CompType,
    edge_list: Vec<Vec<usize>>,
    mapping: Vec<usize>,
}
impl DefaultComp {
    fn new(t: CompType, edge_list: Vec<Vec<usize>>, mapping: Vec<usize>) -> Self {
        Self{
            comp_typ: t,
            edge_list: edge_list,
            mapping: mapping,
        }
    }
}

pub struct DefaultGraph {
    pub components: Vec<DefaultComp>,
    pub default_edges: Vec<Vec<usize>>,
    pub mapping: Vec<Option<usize>>,
}
impl DefaultGraph {
    pub fn new(delta: &Vec<HashMap<char, usize>>) -> Self {
        let mut new = Self{
            components: Vec::new(),
            default_edges: compute_default_graph(delta),
            mapping: vec![None; delta.len()],
        };
        new.compute_default_components();
        return new;
    }

    fn compute_default_components(&mut self) { //TODO think of a representation of default components
        let roots: Vec<usize> = find_roots(&self.default_edges);
        let rev_default_edges = reverse_edges(&self.default_edges);
    
        for root in roots.iter() {
            let indep = self.calc_independent(&rev_default_edges, *root);
            self.components.push(indep);
        }
    }

    fn calc_independent(&mut self, rev_edges: &Vec<Vec<usize>>, root: usize) -> DefaultComp {
        let mut return_list: Vec<Vec<usize>> = vec![Vec::new()];
        let mut comp_mapping: Vec<usize> = Vec::new();
        self.mapping[root] = Some(0);
        comp_mapping.push(root);
        let mut queue: Vec<usize> = vec![root];

        while !queue.is_empty() {
            let p = queue.pop().unwrap();
            let mut edges: Vec<usize> = Vec::new();
            for q in rev_edges[p].iter() {
                queue.push(*q);
                if let Some(x) = self.mapping[*q] {
                    edges.push(x);
                }
                else {
                    self.mapping[*q] = Some(comp_mapping.len());     
                    edges.push(comp_mapping.len());
                    comp_mapping.push(*q);
                    return_list.push(Vec::new());
                }
            }
            if let Some(x) = self.mapping[p] {
                return_list[x] = edges;
            }
        }

        return DefaultComp::new(CompType::Independent, return_list, comp_mapping);
    }
}

//TODO Testing
fn compute_default_graph(delta: &Vec<HashMap<char, usize>>) -> Vec<Vec<usize>> {
    let lq = compute_longest_pairs(delta);
    let mut default_edges: Vec<Vec<usize>> = vec![Vec::new();delta.len()];
    for q in 0..lq.len() {
        match lq[q].get(0) {
            Some((a, _l)) => {
                let tmp: usize = *(delta[q].get(a).unwrap());
                default_edges[q].push(tmp);
            },
            None => continue,
        };
    }
    return default_edges;
}


#[inline]
fn find_roots(default_edges: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut roots: Vec<usize> = Vec::new();
    for q in 0..default_edges.len() {
        if default_edges[q].len() == 0 {
            roots.push(q);
        }
    }
    return roots;
}

fn reverse_edges(edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{   // TODO outsource to graph_alg.rs
    let mut rev_edges: Vec<Vec<usize>> = vec![Vec::new(); edges.len()];

    for u in 0..edges.len() {
        for v in edges[u].iter() {
            rev_edges[*v].push(u);
        }
    }
    return rev_edges;
}

fn main() {
    println!("Hello World");
}