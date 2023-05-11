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
    pub rev_default_edges: Vec<Vec<usize>>,
    pub comp_idx: Vec<Option<usize>>,   //maps the node to the the component on wich it lies
    pub mapping: Vec<Option<usize>>,    //maps the node of the graph to the index in the component
}
impl DefaultGraph {
    pub fn new(delta: &Vec<HashMap<char, usize>>) -> Self {
        let mut new = Self{
            components: Vec::new(),
            default_edges: compute_default_graph(delta),
            rev_default_edges: Vec::new(),
            comp_idx: vec![None; delta.len()],
            mapping: vec![None; delta.len()],
        };
        new.rev_default_edges = reverse_edges(&new.default_edges);
        new.compute_default_components();
        return new;
    }

    fn compute_default_components(&mut self) { //TODO think of a representation of default components
        let roots: Vec<usize> = find_roots(&self.default_edges);
    
        for root in roots.iter() {
            let indep = self.calc_independent(*root);
            self.components.push(indep);
        }
        let mut visited: Vec<bool> = vec![false; self.default_edges.len()];
        for p in 0..self.comp_idx.len() {
            match self.mapping[p] {
                Some(_) => continue,
                None => self.find_cycle(p, &mut visited),
            };
        }
    }

    fn calc_independent(&mut self, root: usize) -> DefaultComp {
        let mut return_list: Vec<Vec<usize>> = vec![Vec::new()];
        let mut comp_mapping: Vec<usize> = Vec::new();

        self.mapping[root] = Some(0);       //TODO group theses operation inside a method
        comp_mapping.push(root);
        self.comp_idx[root] = Some(self.components.len());

        let mut queue: Vec<usize> = vec![root];

        while !queue.is_empty() {
            let p = queue.pop().unwrap();
            let mut edges: Vec<usize> = Vec::new();
            for q in self.rev_default_edges[p].iter() {
                queue.push(*q);
                if let Some(x) = self.mapping[*q] {
                    edges.push(x);
                }
                else {
                    self.mapping[*q] = Some(comp_mapping.len());     //same here
                    edges.push(comp_mapping.len());
                    comp_mapping.push(*q);
                    self.comp_idx[*q] = Some(self.components.len());
                    return_list.push(Vec::new());
                }
            }
            if let Some(x) = self.mapping[p] {
                return_list[x] = edges;
            }
        }

        return DefaultComp::new(CompType::Independent, return_list, comp_mapping);
    }

    fn find_cycle(&mut self, mut current: usize, visited: &mut Vec<bool>) {
        let mut next = self.default_edges[current][0];
        while !visited[next] {
            visited[current] = true;
            current = next;
            next = self.default_edges[current][0];
        }
        let cycle = self.calc_cycle(next);
        for p in cycle.mapping.iter() {
            self.calc_connected(*p);
        }
        for p in cycle.mapping.iter() {
            self.mapping[*p] = Some(self.components.len());
        }
        self.components.push(cycle);
    }

    fn calc_cycle(&mut self, start: usize) -> DefaultComp {
        let mut current = start;
        let mut next = self.default_edges[current][0];
        let mut idx: usize = 0;
        let mut mapping: Vec<usize> = vec![current];
        self.mapping[current] = Some(idx);
        self.comp_idx[current] = Some(self.components.len());
        
        while next != start {
            current = next;
            idx += 1;
            mapping.push(current);
            self.mapping[current] = Some(idx);
            self.comp_idx[current] = Some(self.components.len());

            next = self.default_edges[current][0];
        }
        return DefaultComp::new(CompType::Cycle, Vec::new(), mapping);
    }

    fn calc_connected(&mut self, root: usize) {     //TODO clean up code
        if self.rev_default_edges[root].len() == 1 {
            return;
        }
        let mut edge_list: Vec<Vec<usize>> = vec![Vec::new()];
        let mut comp_mapping: Vec<usize> = Vec::new();
        comp_mapping.push(root);
        let mut queue: Vec<usize> = vec![root];

        while !queue.is_empty() {
            let p = queue.pop().unwrap();
            let mut edges: Vec<usize> = Vec::new();
            for q in self.rev_default_edges[p].iter() {
                println!("from {} to {}", p, *q);
                match self.comp_idx[*q] {
                    Some(_) => continue,
                    None => {
                        println!("mapping of {}: {:?}", *q, self.mapping[*q]);
                        queue.push(*q);
                        match self.mapping[*q] {
                            Some(x) => edges.push(x),
                            None => {
                                self.mapping[*q] = Some(comp_mapping.len());     //same here
                                edges.push(comp_mapping.len());
                                println!("edges to be pushed: {:?}", edges);
                                comp_mapping.push(*q);
                                self.comp_idx[*q] = Some(self.components.len());
                                edge_list.push(Vec::new());
                            },
                        };
                    }
                };
                
            }
            if p == root {
                edge_list[0] = edges;
            }
            else if let Some(x) = self.mapping[p] {
                edge_list[x] = edges;
            }
        }
        self.components.push(DefaultComp::new(CompType::Connected, edge_list, comp_mapping));
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