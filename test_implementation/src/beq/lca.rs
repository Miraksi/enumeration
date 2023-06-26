mod range_min;

use range_min::RMQ;

pub struct LCA {
    euler_tour: Vec<u32>,
    idx_map: Vec<usize>,    // stores the index of the Node of the euler tour to the original node
    last_occ: Vec<usize>,   // stores the last occurrence of out inital node in the tour
    rmq: RMQ,
}

impl LCA {
    pub fn new(parent: &Vec<usize>, children: &Vec<Vec<usize>>, root: usize) -> Self {
        let mut tour: Vec<u32> = Vec::new();
        let mut map: Vec<usize> = Vec::new();
        let mut last_occ: Vec<usize> = vec![0; parent.len()];
        euler_dfs(&mut tour, &mut map, &mut last_occ, &parent, &children, root, 0);

        Self {
            euler_tour: tour.clone(),
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
    tour: &mut Vec<u32>, 
    map: &mut Vec<usize>, 
    last_occ: &mut Vec<usize>, 
    parent: &Vec<usize>, 
    children: &Vec<Vec<usize>>, 
    root: usize, 
    height: u32) 
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