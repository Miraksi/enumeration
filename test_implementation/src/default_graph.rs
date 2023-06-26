mod longest_path;

use std::collections::HashMap;
use crate::{level_ancestor::LevelAncestor, beq::Bottleneck};
use longest_path::compute_longest_pairs;
use crate::beq::{lca::LCA, cartesian::cartesian_on_list};

pub enum CompType {
    Ind(Tree),
    Con(Tree),
    Cyc(Cycle),
}

pub struct Tree {
    edge_list: Vec<Vec<usize>>, 
    depth: Vec<usize>,
    pub la: LevelAncestor,
    pub beq: Bottleneck,
    pub mapping: Vec<usize>,    //map for internal to external;
}
impl Tree {
    fn new(edge_list: Vec<Vec<usize>>, mapping: Vec<usize>, lq: &Vec<Vec<(char, u32)>>) -> Self {
        let mut depth: Vec<usize> = vec![0; edge_list.len()];
        let parent = compute_parents(&edge_list);
        calc_depth(&edge_list, &mut depth, 0, 0);
        let weights = weigh_tree(&depth, &mapping, lq);  //is needed for PathMaxNode
        let (beq_parent, beq_children, beq_weights) = to_beq_tree(&parent, &edge_list, &weights);
        Self {
            edge_list: edge_list.clone(),
            depth: depth,
            la: LevelAncestor::new(&parent, &edge_list, 0),
            beq: Bottleneck::new(beq_parent, beq_children, beq_weights, edge_list.len()),
            mapping: mapping,
        }
    }  
}
fn weigh_tree(depth: &Vec<usize>, mapping: &Vec<usize>, lq: &Vec<Vec<(char, u32)>>) -> Vec<i64> {
    let mut weight: Vec<i64> = Vec::new();
    for i in 0..mapping.len() {    // for the root of independent trees, there is no w_q since 
        if lq[mapping[i]].len() < 2 {
            weight[i] = 0;
        }
        else {
            let l = lq[mapping[i]][1].1;
            weight[i] = l as i64;
        }
        weight[i] = weight[i] - depth[i] as i64;
    }
    return weight;
}

fn to_beq_tree(parent: &Vec<usize>, children: &Vec<Vec<usize>>, weights: &Vec<i64>) -> (Vec<usize>, Vec<Vec<usize>>, Vec<Vec<i64>>) {
    let mut beq_parent: Vec<usize> = vec![0; parent.len()];
    let mut beq_children: Vec<Vec<usize>> = vec![Vec::new(); children.len()];
    let mut beq_weights: Vec<Vec<i64>> = vec![Vec::new(); children.len()*3];

    for i in 0..parent.len() {
        let upper = beq_parent.len();
        let lower = beq_parent.len() + 1;
        beq_parent[i] = upper;
        beq_children[i] = vec![lower];
        beq_parent.push(parent[i]);
        beq_children.push(vec![i]);
        beq_parent.push(i);
        beq_children.push(children[i].clone());
        beq_weights[upper] = vec![-weights[i]];
        beq_weights[i] = vec![-weights[i]];
        beq_weights[lower] = vec![i64::MAX; beq_children[lower].len()];
    }
    return (beq_parent, beq_children, beq_weights);
}
fn calc_depth(edge_list: &Vec<Vec<usize>>, depth: &mut Vec<usize>, curr: usize, curr_depth: usize) {
    depth[curr] = curr_depth;
    for next in edge_list[curr].iter() {
        calc_depth(edge_list, depth, *next, curr_depth + 1);
    }
}

pub struct Cycle {
    pub nodes: Vec<usize>,
    pub lca: LCA,   // rmq over length 2m
}
impl Cycle {
    fn new(nodes: Vec<usize>, lq: &Vec<Vec<(char, u32)>>) -> Self {
        let weights = weigh_cycle(&nodes, lq);
        let (c_root, c_parent, c_children) = cartesian_on_list(&weights);
        Self {
            nodes: nodes,
            lca: LCA::new(&c_parent, &c_children, c_root),
        }
    }
}

// TODO I DONT TRUST THIS WEIGHING YET
// weighs cycle like in Paper, but negates weights, to get range max and not range min
fn weigh_cycle(nodes: &Vec<usize>, lq: &Vec<Vec<(char, u32)>>) -> Vec<i64> {
    let len = nodes.len();
    let mut weights: Vec<i64> = Vec::new();
    for i in 0..2*len {
        if let Some((_,x)) = lq[nodes[i % len]].get(1) {
            weights.push(i as i64 - *x as i64);
        }
        else {
            weights.push(i as i64);
        }
    }
    return weights;
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

    fn calc_independent(&mut self, root: usize) -> Tree {
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

        return Tree::new(return_list, comp_mapping, &self.lq);
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
        return Cycle::new(mapping, &self.lq);
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
        let tmp = Tree::new(edge_list, comp_mapping, &self.lq);
        self.components.push(CompType::Con(tmp));
    }

    pub fn get_depth(&self, i: usize) -> usize {
        match &self.components[self.comp_idx[i].unwrap()] {
            CompType::Ind(x) => x.depth[self.mapping[i].unwrap()],
            CompType::Con(x) => x.depth[self.mapping[i].unwrap()],
            CompType::Cyc(_) => panic!("no depth defined for cycles"),
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

fn compute_parents(edge_list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut parent: Vec<usize> = vec![0; edge_list.len()];
    for i in 0..edge_list.len() {
        for child in edge_list[i].iter() {
            parent[*child] = i;
        }
    }
    return parent;
}

fn main() {
    println!("Hello World");
}