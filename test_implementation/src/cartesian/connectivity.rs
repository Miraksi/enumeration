use std::slice::Iter;
use std::collections::HashMap;

use even_shil::EvenShil;
mod even_shil;
// TODO CLEAN THE COOODDEEEE

fn log_floor(x: u32) -> u32 {   // TODO outsource this code into a module
    return u32::BITS - x.leading_zeros() - 1;
}

#[derive(Debug, PartialEq, Eq, Hash)]       //TODO check, if Hash needs to be derived
enum CompID {
    Macro(usize),
    Micro(usize, usize)
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Component {
    pub parent: Option<usize>,
    pub side: Option<Side>,
}
impl Component {
    fn new() -> Self {
        Self{
            parent: None,
            side: None,
        }
    }
}


#[derive(Clone,Debug)]
pub struct Node {
    parent: usize,
    children: Vec<usize>,
}
impl Node {
    pub fn new(parent: usize, children: Vec<usize>) -> Self {
        Node{
            parent: parent,
            children: children,
        }
    }
}

//TODO export into its own module
#[derive(Debug)]
struct LinkedListSet {
    sets: Vec<usize>,
    size: Vec<usize>,
    tdeg: Vec<usize>,
    next: Vec<Option<usize>>,
    last: Vec<usize>,
}
impl LinkedListSet {
    fn new() -> Self {
        Self {
            sets: Vec::new(),
            size: Vec::new(),
            tdeg: Vec::new(),
            next: Vec::new(),
            last: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Cluster {
    root: usize,
    nodes: Vec<usize>,
    bounds: Vec<usize>,
    edge_map: HashMap<(usize, usize), usize>,
    current_edges: usize,
    root_path: HashMap<usize, usize>,
}
impl Cluster {
    fn new(root: usize, nodes: Vec<usize>, bounds: Vec<usize>) -> Self {
        let size = nodes.len();
        Self{
            root: root,
            nodes: nodes,
            bounds: bounds,
            edge_map: HashMap::new(),
            current_edges: (1 << size-1) -1,
            root_path: HashMap::from([(root, 0)]),
        }
    }
}

#[derive(Debug)]
pub struct Connectivity {
    pub root: usize,
    pub nodes: Vec<Node>,
    pub edge_map: HashMap<(usize,usize),(usize,usize)>, // maps edges of input tree to internal binary tree edges if they were modified
    pub clusters: Vec<Cluster>,
    pub cluster_mapping: Vec<Option<usize>>,
    pub root_path: Vec<usize>,
    pub even_shil: EvenShil,
    pub macro_mapping: Vec<Option<usize>>,
    pub comp_list: Vec<Component>,
    comp_mapping: HashMap<CompID, usize>,  //TODO rewrite this to not need a HashMap
}
impl Connectivity {
    pub fn new(parent: &Vec<usize>, children: &Vec<Vec<usize>>, root: usize) -> Self {
        let mut nodes: Vec<Node> = compute_node_list(parent, children, root);
        let mapping = normalize(&mut nodes, root);
        let n = nodes.len();
        let z = log_floor(n as u32);

        let mut tmp = Connectivity{
            root: root,
            nodes: nodes,
            edge_map: mapping,
            clusters: Vec::new(),
            cluster_mapping: vec![None; n],
            root_path: vec![0; n],
            even_shil: EvenShil::new(Vec::new()),
            macro_mapping: vec![None; n],
            comp_list: vec![Component::new()],
            comp_mapping: HashMap::from([(CompID::Macro(0), 0)]),
        };
        let (_, list) = tmp.cluster(root, z as usize, LinkedListSet::new());
        tmp.collect_cluster(0, &list);
        tmp.fill_clusters();
        tmp.even_shil = EvenShil::new(tmp.build_macro_forest());
        return tmp;
    }

    pub fn connected(&self, u: usize, v: usize) -> bool {
        let (u,v) = self.get_bin_edge(u,v);

        if self.cluster_mapping[u] == self.cluster_mapping[v] {
            return self.micro_connected(self.cluster_mapping[u].unwrap(), u, v);
        }
        let u_bounds = self.get_connected_bounds(u);
        let v_bounds = self.get_connected_bounds(v);

        for i in u_bounds.iter() {
            for j in v_bounds.iter() {
                if self.macro_connected(*i,*j) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn delete(&mut self, u: usize, v: usize) {
        let (u,v) = self.get_bin_edge(u,v);

        if self.cluster_mapping[u] != self.cluster_mapping[v] {
            self.macro_delete(u,v);
            return;
        }
        else {
            let cluster = self.cluster_mapping[u].unwrap();
            self.micro_delete(cluster, u, v);
            if self.clusters[cluster].bounds.len() == 2 {
                let fst = self.clusters[cluster].bounds[0];
                let snd = self.clusters[cluster].bounds[1];
                if !self.micro_connected(cluster, fst, snd) {
                    self.macro_delete(fst, snd);
                }
            }
        }
        self.add_component(u,v);
    }

    pub fn get_comp_id(&self, u: usize) -> CompID {
        let cluster = self.cluster_mapping[u].unwrap();
        for bound in self.clusters[cluster].bounds.iter() {
            if self.connected(u, *bound) {
                let macro_node = self.macro_mapping[*bound].unwrap();
                return CompID::Macro(self.even_shil.get_component(macro_node));
            }
        }
        let identifier = self.root_path[u] & !self.clusters[cluster].current_edges;     
        return CompID::Micro(cluster, identifier); //this should be unique for every connected component
    }

    pub fn get_comp_idx(&self, u: usize) -> usize {
        //println!("for u: {} we have CompID: {:?}", u, self.get_comp_id(u));
        return *self.comp_mapping.get(&self.get_comp_id(u)).unwrap();
    }

    fn macro_connected(&self, u: usize, v: usize) -> bool {
        let u = self.macro_mapping[u].unwrap();
        let v = self.macro_mapping[v].unwrap();
        return self.even_shil.connected(u, v);
    }

    fn macro_delete(&mut self, u: usize, v: usize) {
        let u = self.macro_mapping[u].unwrap();
        let v = self.macro_mapping[v].unwrap();
        self.even_shil.delete(u,v);
    }

    fn micro_connected(&self, idx: usize, u: usize, v: usize) -> bool {
        let cluster = &self.clusters[idx];
        let u_rp = self.root_path[u];
        let v_rp = self.root_path[v];
        return ((u_rp ^ v_rp) & !cluster.current_edges) == 0;
    }

    fn micro_delete(&mut self, idx: usize, u: usize, v: usize) {
        let mut parent = u;
        let mut child = v;
        if self.get_parent(u) == v {
            parent = v;
            child = u;
        }
        let edge_idx = self.clusters[idx].edge_map.get(&(parent, child)).unwrap();
        if self.clusters[idx].current_edges & 1 << *edge_idx > 0 {
            self.clusters[idx].current_edges -= 1 << *edge_idx;
        }
    }

    fn get_connected_bounds(&self, u: usize) -> Vec<usize> {
        let i = self.cluster_mapping[u].unwrap();
        let mut list: Vec<usize> = Vec::new();
        for bound in self.clusters[i].bounds.iter() {
            if self.micro_connected(i, u, *bound) {
                list.push(*bound);
            }
        }
        return list;
    }

// algorithm from: Ambivalent data structures 
    fn cluster(&mut self, root: usize, z: usize, mut links: LinkedListSet) -> (usize, LinkedListSet) {
        let v = links.sets.len();
        links.sets.push(root);
        links.size.push(1);
        links.tdeg.push(self.nodes[root].children.len());
        if root != self.root {
            links.tdeg[v] += 1;
        }
        links.next.push(None);
        links.last.push(v);

        for i in self.nodes[root].children.clone().iter() {
            let (w, tmp) = self.cluster(*i, z, links);
            links = tmp;
            //println!("root: {root}, tdeg[v]: {}, size[v]: {}  \tchild: {}, tdeg[w]: {}, size[w]: {}",links.tdeg[v], links.size[v], links.sets[w], links.tdeg[w], links.size[w]);
            if links.tdeg[v] + links.tdeg[w] <= 4 && links.size[v] + links.size[w] <= z {
                links.tdeg[v] = (links.tdeg[v] + links.tdeg[w]) - 2;
                links.size[v] += links.size[w];
                links.next[links.last[v]] = Some(w);
                links.last[v] = links.last[w];
                //println!("root: {root}, next[v]: {:?} last[v]: {}", links.sets[links.next[v].unwrap()], links.sets[w]);
            }
            else {
                //println!("for root {root}, collect: {}",links.sets[w]);
                self.collect_cluster(w, &links);
            }
        }
        return (v, links);
    }

    fn collect_cluster(&mut self, mut i: usize, links: &LinkedListSet) {
        let cluster_idx = self.clusters.len();
        let mut nodes: Vec<usize> = Vec::new();
        loop {
            nodes.push(links.sets[i]);
            self.cluster_mapping[links.sets[i]] = Some(cluster_idx);
            match links.next[i] {
                None => break,
                Some(j) => i = j,
            };
        }
        let mut root = nodes[0];
        for n in nodes.iter() {
            if self.cluster_mapping[*n] != self.cluster_mapping[self.get_parent(*n)] {
                root = *n;
            }
        }
        let bounds = self.compute_bounds(&nodes);
        self.clusters.push(Cluster::new(root, nodes, bounds));
    }

    fn compute_bounds(&mut self, nodes: &Vec<usize>) -> Vec<usize> {
        let mut bounds: Vec<usize> = Vec::new();
        for v in nodes.iter() {
            let cluster = self.cluster_mapping[*v];
            if cluster != self.cluster_mapping[self.get_parent(*v)] {
                bounds.push(*v);
                continue;
            }
            for w in self.get_children(*v) {
                if cluster != self.cluster_mapping[*w] {
                    bounds.push(*v);
                    break;
                }
            }
        }
        return bounds;
    }

    fn fill_clusters(&mut self) {
        for i in 0..self.clusters.len() {
            self.fill_cluster_map(i);
            self.fill_rootpath(i);
        }
    }

    fn fill_cluster_map(&mut self, i: usize) {
        let mut count: usize = 0;
        for j in 0..self.clusters[i].nodes.len() {
            let u = self.clusters[i].nodes[j];
            for k in 0..self.nodes[u].children.len() {
                let v = self.nodes[u].children[k];
                if self.cluster_mapping[u].unwrap() == self.cluster_mapping[v].unwrap() {
                    self.clusters[i].edge_map.insert((u,v), count);
                    count += 1;
                }
            }
        }
    }

    fn fill_rootpath(&mut self, i: usize) {
        let root = self.clusters[i].root;
        let mut queue: Vec<usize> = Vec::new();
        for child in self.get_children(root) {
            if self.cluster_mapping[root] == self.cluster_mapping[*child] {
                queue.push(*child);
            }
        }

        while !queue.is_empty() {
            let current = queue.pop().unwrap();
            let parent = self.get_parent(current);
            let parent_path: usize = self.root_path[parent];
            let edge_idx: usize = *self.clusters[i].edge_map.get(&(parent,current)).unwrap();
            self.root_path[current] = parent_path + (1 << edge_idx);

            for child in self.get_children(current) {
                if self.cluster_mapping[current] == self.cluster_mapping[*child] {
                    queue.push(*child);
                }
            }
        }
    }

    fn build_macro_forest(&mut self) -> Vec<even_shil::Node>{
        let mut forest: Vec<even_shil::Node> = Vec::new();

        for i in 0..self.clusters.len() {
            let fst_bound = self.clusters[i].bounds[0];
            self.bound_to_macro(fst_bound, &mut forest);
            if self.clusters[i].bounds.len() == 2 {
                let snd_bound = self.clusters[i].bounds[1];
                self.bound_to_macro(snd_bound, &mut forest);
                self.add_macro_node(fst_bound, snd_bound, &mut forest);
            }
        }
        return forest;
    }

    fn bound_to_macro(&mut self, v: usize, forest: &mut Vec<even_shil::Node>) {
        let cluster = self.cluster_mapping[v];

        for i in 0..self.nodes[v].children.len() {
            let w = self.nodes[v].children[i];
            if cluster != self.cluster_mapping[w] {
                self.add_macro_node(v, w, forest);
            }
        }
    }

    fn add_macro_node(&mut self, u: usize, v: usize, forest: &mut Vec<even_shil::Node>) {
        if self.macro_mapping[u] == None {
            let idx = forest.len();
            self.macro_mapping[u] = Some(idx);
            forest.push(even_shil::Node::new(Vec::new()));
        }
        if self.macro_mapping[v] == None {
            let idx = forest.len();
            self.macro_mapping[v] = Some(idx);
            forest.push(even_shil::Node::new(Vec::new()));
        }
        let u_idx = self.macro_mapping[u].unwrap();
        let v_idx = self.macro_mapping[v].unwrap();
        forest[u_idx].adjacent.push(v_idx);
        forest[v_idx].adjacent.push(u_idx);
    }

    fn add_component(&mut self, u: usize, v: usize) {
        let u_comp = self.get_comp_id(u);
        let v_comp = self.get_comp_id(v);
        let len = self.comp_list.len();
        
        if !self.comp_mapping.contains_key(&u_comp) {
            self.comp_mapping.insert(u_comp, len);
        }
        if !self.comp_mapping.contains_key(&v_comp) {
            self.comp_mapping.insert(v_comp, len);
        }
        self.comp_list.push(Component::new());
    }

    fn get_parent(&self, node: usize) -> usize {
        return self.nodes[node].parent;
    }

    fn get_children(&self, node: usize) -> Iter<usize> {
        return self.nodes[node].children.iter();
    }

    // expects an edge in the original input tree
    fn get_bin_edge(&self, u: usize, v: usize) -> (usize, usize) {
        if let Some(x) = self.edge_map.get(&(u,v)) {
            return *x;
        }
        match self.edge_map.get(&(v,u)) {
            Some(x) => return *x,
            None => return (u,v),
        };
    }
}

//export to graph mod
pub fn compute_node_list(parent: &Vec<usize>, children: &Vec<Vec<usize>>, root: usize) -> Vec<Node> {   
    let mut list: Vec<Node> = Vec::new();
    for i in 0..parent.len() {
        let node = Node::new(parent[i], children[i].clone());
        list.push(node);
    }
    list[root].parent = root;   //Sets the parent of the root to always be the root
    return list;
}

// brings the tree into binary tree shape
pub fn normalize(nodes: &mut Vec<Node>, root: usize) -> HashMap<(usize,usize),(usize, usize)> {  
    let mut edge_map: HashMap<(usize,usize),(usize, usize)> = HashMap::new();
    let mut queue: Vec<usize> = vec![root];
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        for child in nodes[current].children.iter() {
            queue.push(*child);
        }
        if nodes[current].children.len() <= 2 {     //change this to get binary trees
            continue;
        }
        let first_added = nodes.len();
        for i in 1..nodes[current].children.len() {
            let child = nodes[current].children[i];
            edge_map.insert((current, child), (nodes.len(), child));
            nodes[child].parent = nodes.len();
            let tmp = Node::new(nodes.len()-1, vec![child, nodes.len()+1]);
            nodes.push(tmp);
        }
        nodes[first_added].parent = current;
        let last = nodes.len() - 1;
        nodes[last].children.pop();
        nodes[current].children = vec![nodes[current].children[0], first_added];
    }
    return edge_map;
}

fn main() {
    
    let mut children: Vec<Vec<usize>> = Vec::new();

    children.push(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());
    children.push(Vec::new());

    let parent = vec![0;children.len()];   //TODO test for cases with more than one boundary node
    
    let mut con = Connectivity::new(&parent, &children, 0);
    println!("Nodes: {:?}\n", con.nodes);
    println!("Clusters: {:?}\n", con.clusters);
    println!("ClusterMapping: {:?}\n", con.cluster_mapping);
    println!("connected(0,16): {}", con.connected(0,16));
    println!("connected(0,4): {}", con.connected(0,4));
    con.delete(0,1);
    println!("delete(0,1)");
    println!("connected(0,16): {}", con.connected(0,16));
    println!("connected(0,4): {}", con.connected(0,4));
    con.delete(0,4);
    println!("delete(0,4)");
    println!("connected(0,4): {}", con.connected(0,4));

}