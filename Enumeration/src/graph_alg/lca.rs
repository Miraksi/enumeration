pub mod range_min;

use range_min::RMQ;

/// A data-structure for answering LCA queries
///
/// # Complexity
/// Precomutation in O(n) and queries in O(1) time
///
/// # Sources
/// used <https://cp-algorithms.com/graph/lca_farachcoltonbender.html> as reference
pub struct LCA {
    idx_map: Vec<usize>,    // stores the index of the Node of the euler tour to the original node
    last_occ: Vec<usize>,   // stores the last occurrence of out inital node in the tour
    rmq: RMQ<usize>,
}

impl LCA {
    pub fn new(parent: &Vec<usize>, children: &Vec<Vec<usize>>, root: usize) -> Self {
        let mut tour: Vec<usize> = Vec::new();
        let mut map: Vec<usize> = Vec::new();
        let mut last_occ: Vec<usize> = vec![0; parent.len()];
        if parent.len() == 0 {
            return LCA::new(&vec![0], &vec![vec![]], 0);
        }
        euler_dfs(&mut tour, &mut map, &mut last_occ, &parent, &children, root, 0);

        Self {
            idx_map: map,
            last_occ: last_occ,
            rmq: RMQ::new(tour),
        }
    }

    pub fn get(&self, i: usize, j: usize) -> usize {
        let a = self.last_occ[i];
        let b = self.last_occ[j];
        return self.idx_map[self.rmq.get(a,b)];
    }
}

fn euler_dfs(
    tour: &mut Vec<usize>, 
    map: &mut Vec<usize>, 
    last_occ: &mut Vec<usize>, 
    parent: &Vec<usize>, 
    children: &Vec<Vec<usize>>, 
    root: usize, 
    height: usize) 
    {
    last_occ[root] = tour.len();
    tour.push(height);
    map.push(root);
    
    for child in children[root].clone().iter() {
        euler_dfs(tour, map, last_occ, parent, children, *child, height + 1);
        last_occ[root] = tour.len();
        tour.push(height);
        map.push(root);
    }
}