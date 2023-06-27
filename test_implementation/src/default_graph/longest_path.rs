use tarjan::Tarjan;
use rdxsort::*; //TODO test if its actually fast
use std::collections::HashMap;
use crate::weight::{Weight, Weight::*};

mod tarjan;


const INFTY: u32 = 4294967295;

#[derive(Clone, PartialEq)] // needed for list-comprehension and == operator
enum Color {
    WHITE,
    RED,
}

pub fn compute_longest_pairs(delta: &Vec<HashMap<char, usize>>) -> Vec<Vec<(char,Weight)>> { //Lq
    let edges = compute_edge_list(&delta);
    let pi = compute_pi(&edges);
    let mut triple_list: Vec<(u32, char, usize)> = Vec::new(); // (pi(q'), a, q) instead of (q, pi(q'), a)
    for q in 0..delta.len() {
        for (a, q_next) in delta[q].iter() {
            triple_list.push((pi[*q_next], *a, q));
        }
    }
    triple_list.rdxsort();
    let mut longest_pairs: Vec<Vec<(char,Weight)>> = vec![Vec::new(); delta.len()];
    for (length, a, q) in triple_list.iter().rev() {
        let tmp = match *length {
            INFTY => (*a, Inf),
            _ => (*a, Val((length + 1) as i64)),
        };
        longest_pairs[*q].push(tmp);
    }
    return longest_pairs;
}

fn compute_edge_list(delta: &Vec<HashMap<char, usize>>) -> Vec<Vec<usize>> {
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); delta.len()];
    for p in 0..delta.len() {
        for (_a, q) in delta[p].iter() {
            edges[p].push(*q);
        }
    }
    return edges;
}

fn compute_pi(edges: &Vec<Vec<usize>>) -> Vec<u32> {
    let n = edges.len();
    let rev_edges = reverse_edges(edges);
    let mut color = vec![Color::WHITE;n];
    let connected = Tarjan::new(edges).tarjan();
    let mut queue: Vec<usize> = Vec::new();
    let mut pi: Vec<u32> = vec![0;n];

    for list in connected.iter() {
        for node in list.iter() {
            color[*node] = Color::RED;
            queue.push(*node);
        }
    }
    while !queue.is_empty() {
        let p = queue.pop().unwrap();
        pi[p] = INFTY;

        for &s in rev_edges[p].iter() {
            if color[s] == Color::WHITE  {
                color[s] = Color::RED; // has to be checked
                queue.push(s);
            }
        }
    }

    let acyc_edges = compute_acyclic_graph(edges, &color);
    let topsort = Topsort::new(&acyc_edges).reverse_topsort();
    let rev_edges = reverse_edges(&acyc_edges);

    for u in topsort {
        if color[u] == Color::WHITE {
            for v in rev_edges[u].iter() {
                pi[*v] = match pi[u] {
                    INFTY => INFTY,
                    _ => max(pi[*v], pi[u] + 1),
                };
            }
        }
    }
    // Test
    return pi;
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

fn compute_acyclic_graph(edges: &Vec<Vec<usize>>, color: &Vec<Color>) -> Vec<Vec<usize>> {
    let mut acyc_edges: Vec<Vec<usize>> = vec![Vec::new();edges.len()];
    for u in 0..edges.len() {
        if color[u] == Color::WHITE {
            for v in edges[u].iter() {
                if color[*v] == Color::WHITE {
                    acyc_edges[u].push(*v);
                }
            }
        } 
    }
    return acyc_edges;
}

//Topsort from teamreferences
struct Topsort<'a> {
    edges: &'a Vec<Vec<usize>>,
    used: Vec<bool>,
    topsort: Vec<usize>,
}

impl <'a> Topsort<'a> {
    fn new(edges: &'a Vec<Vec<usize>>) -> Self {
        let n = edges.len();
        Self {
            edges: edges,
            used: vec![false;n],
            topsort: Vec::new(),
        }
    }
    fn reverse_topsort(&mut self) -> Vec<usize> {    
        let n = self.edges.len();
        for i in 0..n {
            if !self.used[i] {
                self.dfs_topsort(i);
            }
        }
        return self.topsort.clone();
    }
    fn dfs_topsort(&mut self, v: usize) {
        self.used[v] = true;
        for u in self.edges[v].iter() {
            if !self.used[*u] {
                self.dfs_topsort(*u);
            }
        }
        self.topsort.push(v);
    }
    
}

fn max(a: u32, b: u32) -> u32 {
    match a < b {
        true => b,
        false => a,
    }
}