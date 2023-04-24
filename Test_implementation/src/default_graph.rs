mod longest_path;

use std::collections::HashMap;
use longest_path::compute_longest_pairs;

enum Default_Component {
    Independent,
    Connected,
    Cycle,
}

fn compute_default_graph(delta: &Vec<HashMap<char, usize>>, lq: &Vec<Vec<(char,u32)>>) -> Vec<Vec<usize>> {
    let default_edges: Vec<Vec<usize>> = vec![Vec::new();delta.len()];
    for q in 0..lq.len() {
        match lq[q].get(0) {
            Some((a, _l)) => default_edges[q].push(delta[q].get(a).unwrap()),
            None => continue;
        };
    }
}

fn compute_default_components(delta: &Vec<HashMap<char, usize>>) { //TODO think of a representation of default components
    let lq = compute_longest_pairs(delta);
    let default_edges = compute_default_graph(delta, &lq);
    let roots: Vec<usize> = find_roots(&default_edges);
    let rev_default_edges = reverse_edges(&default_edges);

}

#[inline]
fn find_roots(default_edges: &Vec<usize>) -> Vec<usize> {
    let mut roots: Vec<usize> = Vec::new();
    for q in 0..default_edges.len() {
        if default_edges[q].len() == 0 {
            roots.push(q);
        }
    }
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