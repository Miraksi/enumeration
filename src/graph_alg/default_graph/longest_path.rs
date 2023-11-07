use crate::graph_alg::{tarjan::Tarjan, toposort::Toposort};
use rdxsort::*; //TODO test if its actually fast
use crate::weight::{Weight, Weight::*};


const INFTY: u32 = 4294967295;

#[derive(Clone, PartialEq)] // needed for list-comprehension and == operator
enum Color {
    WHITE,
    RED,
}

/// Builds the Lq data-structure from Lemma 2 of our main paper.
/// This stores for every state q a sorted list of tuple (a,l), where a is the character with which we
/// transition from q to another state q', and l is the length of the longest path from q with its first 
/// transition beeing to q'.
///
/// # Complexity
/// O(|V| + |E|) instead of O(|V| * sigma) (does this still hold after the changes?)
///
/// # Sources
/// Lemma 2 of 'D. Adamson, F. Manea and P. Gawrychowski. Enumerating Prefix-Closed Regular Languages with Constant Delay'
pub fn compute_longest_pairs(delta: &Vec<Vec<(char, usize)>>) -> Vec<Vec<(char, Weight, usize)>> { //Lq
    let edges = compute_edge_list(&delta);
    let pi = compute_pi(&edges);
    let mut tuple_list: Vec<(u32, char, (usize, usize))> = Vec::new(); // (pi(q'), a, (q, q')) instead of (q, pi(q'), a) to avoid having to use hashmaps. the tupel 
    for q in 0..delta.len() {
        for (a, q_next) in delta[q].iter() {
            tuple_list.push((pi[*q_next], *a, (q, *q_next)));
        }
    }
    tuple_list.rdxsort();
    let mut longest_pairs: Vec<Vec<(char, Weight, usize)>> = vec![Vec::new(); delta.len()];
    for (length, a, (q, q_next)) in tuple_list.iter().rev() {
        let tmp = match *length {
            INFTY => (*a, Inf, *q_next),
            _ => (*a, Val((length + 1) as i64), *q_next),
        };
        longest_pairs[*q].push(tmp);
    }
    return longest_pairs;
}

fn compute_edge_list(delta: &Vec<Vec<(char, usize)>>) -> Vec<Vec<usize>> {
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
    let toposort = Toposort::new(&acyc_edges).reverse_toposort();
    let rev_edges = reverse_edges(&acyc_edges);

    for u in toposort {
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


fn max(a: u32, b: u32) -> u32 {
    match a < b {
        true => b,
        false => a,
    }
}