mod longest_path;

use std::collections::HashMap;
use longest_path::compute_longest_pairs;

#[derive(Debug)]
pub enum CompType {
    Ind(Independent),
    Con(Connected),
    Cyc(Cycle),
}

#[derive(Debug)]
struct Independent {
    edge_list: Vec<Vec<usize>>, 
    depth: Vec<usize>,
    mapping: Vec<usize>,    //map for external to internal
}
impl Independent {
    fn new(edge_list: Vec<Vec<usize>>, mapping: Vec<usize>) -> Self {
        let mut depth: Vec<usize> = vec![0; edge_list.len()];
        calc_depth(&edge_list, &mut depth, 0, 0);
        Self {
            edge_list: edge_list,
            depth: depth,
            mapping: mapping,
        }
    }
}
fn calc_depth(edge_list: &Vec<Vec<usize>>, depth: &mut Vec<usize>, curr: usize, curr_depth: usize) {
    depth[curr] = curr_depth;
    for next in edge_list[curr].iter() {
        calc_depth(edge_list, depth, *next, curr_depth + 1);
    }
}

#[derive(Debug)]
struct Cycle {
    nodes: Vec<usize>,
}
impl Cycle {
    fn new(nodes: Vec<usize>) -> Self {
        Self {
            nodes: nodes,
        }
    }
}

#[derive(Debug)]
struct Connected {
    edge_list: Vec<Vec<usize>>, 
    depth: Vec<usize>,
    mapping: Vec<usize>,    //map for external to internal
}
impl Connected {
    fn new(edge_list: Vec<Vec<usize>>, mapping: Vec<usize>) -> Self {
        let mut depth: Vec<usize> = vec![0; edge_list.len()];
        calc_depth(&edge_list, &mut depth, 0, 0);
        Self {
            edge_list: edge_list,
            depth: depth,
            mapping: mapping,
        }
    }
}

// #[derive(Debug)]
// pub struct DefaultComp {
//     comp_typ: CompType,
//     edge_list: Vec<Vec<usize>>,
//     mapping: Vec<usize>,
// }
// impl DefaultComp {
//     fn new(t: CompType, edge_list: Vec<Vec<usize>>, mapping: Vec<usize>) -> Self {
//         Self{
//             comp_typ: t,
//             edge_list: edge_list,
//             mapping: mapping,
//         }
//     }
// }

pub struct DefaultGraph {
    pub lq: Vec<Vec<(char,u32)>>,
    pub components: Vec<CompType>,
    pub default_edges: Vec<Vec<usize>>,
    pub rev_default_edges: Vec<Vec<usize>>,
    pub comp_idx: Vec<Option<usize>>,   //maps the node to the the component on wich it lies
    pub mapping: Vec<Option<usize>>,    //maps the node of the graph to the index in the component
}
impl DefaultGraph {
    pub fn new(delta: &Vec<HashMap<char, usize>>) -> Self {
        let (lq, default_edges) = compute_default_graph(delta);
        let mut new = Self{
            lq: lq,
            components: Vec::new(),
            default_edges: default_edges,
            rev_default_edges: Vec::new(),
            comp_idx: vec![None; delta.len()],
            mapping: vec![None; delta.len()],
        };
        new.rev_default_edges = reverse_edges(&new.default_edges);
        new.compute_default_components();
        new.weigh_default_nodes(); //prerequisite for PathMaxNode(s,l) 
        return new;
    }

    fn compute_default_components(&mut self) { //TODO think of a representation of default components
        let roots: Vec<usize> = find_roots(&self.default_edges);
    
        for root in roots.iter() {
            let ind = self.calc_independent(*root);
            self.components.push(CompType::Ind(ind));
        }
        let mut visited: Vec<bool> = vec![false; self.default_edges.len()];
        for p in 0..self.comp_idx.len() {
            match self.mapping[p] {
                Some(_) => continue,
                None => self.find_cycle(p, &mut visited),
            };
        }
    }

    fn calc_independent(&mut self, root: usize) -> Independent {
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

        return Independent::new(return_list, comp_mapping);
    }

    fn find_cycle(&mut self, mut current: usize, visited: &mut Vec<bool>) {
        let mut next = self.default_edges[current][0];
        while !visited[next] {
            visited[current] = true;
            current = next;
            next = self.default_edges[current][0];
        }
        let cycle = self.calc_cycle(next);
        for p in cycle.nodes.iter() {
            self.calc_connected(*p);
        }
        for p in cycle.nodes.iter() {
            self.mapping[*p] = Some(self.components.len());
        }
        self.components.push(CompType::Cyc(cycle));
    }

    fn calc_cycle(&mut self, start: usize) -> Cycle {
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
        return Cycle::new(mapping);
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
                match self.comp_idx[*q] {
                    Some(_) => continue,
                    None => {
                        queue.push(*q);
                        match self.mapping[*q] {
                            Some(x) => edges.push(x),
                            None => {
                                self.mapping[*q] = Some(comp_mapping.len());     //same here
                                edges.push(comp_mapping.len());
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
        let tmp = Connected::new(edge_list, comp_mapping);
        self.components.push(CompType::Con(tmp));
    }

    fn weigh_default_nodes(&mut self) {
        for i in 0..self.components.len() {
            match self.components[i] {
                CompType::Ind(_) => self.weigh_ind(i),
                CompType::Cyc(_) => continue,
                CompType::Con(_) => continue,
            }
        }
    }

    fn weigh_ind(&self, idx: usize) {
        let mut weight: Vec<i64> = Vec::new();
        if let CompType::Ind(comp) = &self.components[idx] {
            for i in 0..comp.edge_list.len() {    // for the root of independent trees, there is no w_q since 
                if self.lq[comp.mapping[i]].len() < 2 {
                    weight[i] = 0;
                }
                else {
                    let (_,l) = self.lq[comp.mapping[i]][1];
                    weight[i] = l as i64;
                }
                weight[i] = weight[i] - comp.depth[i] as i64;
            }
        }
        else {
            panic!("weigh_ind recieved a non independent component");
        }
    }
}

//TODO Testing
fn compute_default_graph(delta: &Vec<HashMap<char, usize>>) -> (Vec<Vec<(char,u32)>>, Vec<Vec<usize>>) {
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
    return (lq, default_edges);
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