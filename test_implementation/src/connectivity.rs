use std::slice::Iter;


fn log_floor(x: u32) -> u32 {   // TODO outsource this code into a module
    return u32::BITS - x.leading_zeros() - 1;
}

#[derive(Clone,Debug)]
pub struct Node {
    parent: usize,
    children: Vec<usize>,
    descendants: usize,
}
impl Node {
    pub fn new(parent: usize, children: Vec<usize>) -> Self {
        Node{
            parent: parent,
            children: children,
            descendants: 0,
        }
    }
}

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
    nodes: Vec<usize>,
    bounds: Vec<usize>,
}
impl Cluster {
    fn new(nodes: Vec<usize>, bounds: Vec<usize>) -> Self {
        Self{
            nodes: nodes,
            bounds: bounds,
        }
    }
}

#[derive(Debug)]
pub struct Connectivity {
    pub root: usize,
    pub nodes: Vec<Node>,
    pub clusters: Vec<Cluster>,
    pub mapping: Vec<Option<usize>>,
}
impl Connectivity {
    pub fn new(parent: Vec<usize>, children: Vec<Vec<usize>>, root: usize) -> Self {
        let mut nodes: Vec<Node> = compute_node_list(&parent, children, root);
        normalize(&mut nodes, root);
        let nodes = compute_descendants(nodes, root);
        let n = nodes.len();
        let z = log_floor(n as u32);

        let mut tmp = Connectivity{
            root: root,
            nodes: nodes,
            clusters: Vec::new(),
            mapping: vec![None; n],
        };
        let (_, list) = tmp.cluster(root, z as usize, LinkedListSet::new());
        tmp.collect_cluster(0, &list);
        return tmp;
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
            if links.tdeg[v] + links.tdeg[w] <= 4 && links.size[v] + links.size[w] <= z {
                links.tdeg[v] = (links.tdeg[v] + links.tdeg[w]) - 2;
                links.size[v] += links.size[w];
                links.next[links.last[v]] = Some(w);
                links.last[v] = w;
            }
            else {
                self.collect_cluster(w, &links);
            }
        }             
        return (v, links);
    }

    fn collect_cluster(&mut self, mut current: usize, links: &LinkedListSet) {
        let cluster_idx = self.clusters.len();
        let mut nodes: Vec<usize> = Vec::new();
        loop {
            nodes.push(links.sets[current]);
            self.mapping[links.sets[current]] = Some(cluster_idx);
            match links.next[current] {
                None => break,
                Some(i) => current = i,
            };
        }
        let bounds = self.compute_bounds(&nodes);
        self.clusters.push(Cluster::new(nodes, bounds));
    }

    fn compute_bounds(&self, nodes: &Vec<usize>) -> Vec<usize>{
        let mut bounds: Vec<usize> = Vec::new();
        for v in nodes.iter() {
            let cluster = self.mapping[*v];
            if cluster != self.mapping[self.get_parent(*v)] {
                bounds.push(*v);
                continue;
            }
            for w in self.get_children(*v) {
                if cluster != self.mapping[*w] {
                    bounds.push(*v);
                    break;
                }
            }
        }
        return bounds;
    }
    fn get_parent(&self, node: usize) -> usize {
        return self.nodes[node].parent;
    }

    fn get_children(&self, node: usize) -> Iter<usize> {
        return self.nodes[node].children.iter();
    }

    fn get_descendants(&self, node: usize) -> usize {
        return self.nodes[node].descendants;
    }

}

//export to graph mod
pub fn compute_node_list(parent: &Vec<usize>, children: Vec<Vec<usize>>, root: usize) -> Vec<Node> {   
    let mut list: Vec<Node> = Vec::new();
    for i in 0..parent.len() {
        let node = Node::new(parent[i], children[i].clone());
        list.push(node);
    }
    list[root].parent = root;   //Sets the parent of the root to always be the root
    return list;
}

pub fn normalize(nodes: &mut Vec<Node>, root: usize) {
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
            nodes[child].parent = nodes.len();
            let tmp = Node::new(nodes.len()-1, vec![child, nodes.len()+1]);
            nodes.push(tmp);
        }
        nodes[first_added].parent = current;
        let last = nodes.len() - 1;
        nodes[last].children.pop();
        nodes[current].children = vec![nodes[current].children[0], first_added];
    }
}

fn compute_descendants(mut nodes: Vec<Node>, current: usize) -> Vec<Node> {
    for i in nodes[current].children.clone().iter() {
        nodes = compute_descendants(nodes, *i);
        nodes[current].descendants += 1 + nodes[*i].descendants;
    }
    return nodes;
}
fn main() {
    let mut parent: Vec<usize> = Vec::new();
    let mut children: Vec<Vec<usize>> = Vec::new();

    children.push(vec![1,2]);
    children.push(vec![3,4]);
    children.push(vec![5,6]);
    children.push(vec![7,8]);
    children.push(vec![9,10]);
    children.push(vec![11,12]);
    children.push(vec![13,14]);
    children.push(vec![15,16]);
    children.push(vec![17,18]);
    children.push(vec![19,20]);
    children.push(vec![21,22]);
    children.push(vec![23,24]);
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

    parent = vec![0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11];
    let con = Connectivity::new(parent, children, 0);
    println!("{:?}", con.clusters);
}