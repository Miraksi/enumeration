#[derive(Clone,Debug)]
pub struct Node {
    pub adjacent: Vec<usize>,
}
impl Node {
    pub fn new(adjacent: Vec<usize>) -> Self {
        Node{
            adjacent: adjacent,
        }
    }
}

/// A data-structure that, given a forest, allows dynamic-connectivity queries.
/// Meaning deletion of an edge (u,v) and checking whether two vertecies are still connected.
///
/// # Complexity
/// The preprocessing phase runs in O(n) time, where n is the the number of vertecies in the forest.
/// Deletion runs in O(log n) and checking for connectivity runs in O(1) time.
///
/// # Sources
/// used Wikipedia as reference: <https://en.wikipedia.org/wiki/Dynamic_connectivity>
#[derive(Debug)]
pub struct EvenShil {
    forest: Vec<Node>,
    component: Vec<usize>,
    count: usize,
    visited: Vec<usize>,
    dfs_id: usize,
}
impl EvenShil {     //expects the parent of a root to be itself
    pub fn new(forest: Vec<Node>) -> Self {   
        let n = forest.len();
        let mut tmp = EvenShil{
            forest: forest,
            component: vec![0;n],
            count: 0,
            visited: vec![0;n],
            dfs_id: 1,
        };
        tmp.component = tmp.calc_component();
        return tmp;
    }

    pub fn connected(&self, u: usize, v: usize) -> bool {
        return self.component[u] == self.component[v];
    }

    pub fn delete(&mut self, u: usize, v: usize) { // TODO new version still needs testing
        if self.component[u] != self.component[v] {
            return;
        }

        let mut queue: Vec<usize> = Vec::new();

        if self.is_smaller(u,v) {
            queue.push(u);
            self.dfs_id += 1;
            self.visited[v] = self.dfs_id;
        }
        else {
            queue.push(v);
            self.dfs_id += 1;
            self.visited[u] = self.dfs_id;
        }
        while !queue.is_empty() {
            let current = queue[0];
            self.dfs_step(&mut queue, self.dfs_id);
            self.component[current] = self.count;
        }
        self.count += 1;
    }

    pub fn get_component(&self, u: usize) -> usize {
        return self.component[u];
    }

    fn calc_component(&mut self) -> Vec<usize> {
        let mut visited: Vec<bool> = vec![false; self.forest.len()];
        let mut comp: Vec<usize> = vec![0; self.forest.len()];
    
        for i in 0..self.forest.len() {
            if visited[i] {
                continue;
            }
            let mut queue: Vec<usize> = vec![i];
            while !queue.is_empty() {
                let current = queue.pop().unwrap();
                if !visited[current] {
                    queue.append(&mut self.forest[current].adjacent.clone());
                }
                visited[current] = true;
                comp[current] = self.count;
                
            }
            self.count += 1;
        }
        return comp;
    }

    fn is_smaller(&mut self, u: usize, v: usize) -> bool {
        let mut u_queue: Vec<usize> = vec![u];
        let u_id = self.dfs_id;
        self.visited[v] = u_id;
        self.dfs_id += 1;

        let mut v_queue: Vec<usize> = vec![v];
        let v_id = self.dfs_id;
        self.visited[u] = v_id;
        self.dfs_id += 1;        

        while !u_queue.is_empty() && !v_queue.is_empty() {
            self.dfs_step(&mut u_queue, u_id);
            self.dfs_step(&mut v_queue, v_id);
        }
        return u_queue.is_empty();
    }

    fn dfs_step(&mut self, queue: &mut Vec<usize>, dfs_id: usize) {
        let u = queue.pop().unwrap();
        let comp = self.component[u];
        self.visited[u] = dfs_id;
        for v in self.forest[u].adjacent.iter() {
            if self.visited[*v] == dfs_id || self.component[*v] != comp {
                continue;
            }
            queue.push(*v);
        }
    }
}

