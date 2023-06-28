use std::collections::HashMap;
use crate::default_graph::{DefaultGraph, CompType};
use crate::weight::Weight;

pub struct PathMaxNode {
    pub d_graph: DefaultGraph,
}

impl PathMaxNode {
    pub fn new(delta: &Vec<HashMap<char, usize>>) -> Self {
        Self {
            d_graph: DefaultGraph::new(delta),
        }
    }

    pub fn get(&self, s: usize, l: usize) -> Option<(usize, usize)> {
        println!("get({},{})",s,l);
        let mut best_node = 0;
        let mut d = 0;
        match &self.d_graph.components[self.d_graph.comp_idx[s].unwrap()] {
            CompType::Ind(_) => (best_node, d) = self.get_on_tree(s,l),
            CompType::Con(tree) => {
                let depth = self.d_graph.get_depth(s);
                if  depth > l {
                    (best_node, d) = self.get_on_tree(s, l);
                }
                else {
                    let (on_tree, t_dist) = self.get_on_tree(s, depth);
                    let (on_cycle, c_dist) = self.get_on_cycle(tree.mapping[0], l - depth);
                    if self.d_graph.get_weight(on_tree) < self.d_graph.get_weight(on_cycle) {
                        best_node = on_tree;
                        d = t_dist;
                    }
                    else {
                        best_node = on_cycle;
                        d = depth + c_dist;
                    }
                }
            },
            CompType::Cyc(_) => (best_node, d) = self.get_on_cycle(s, l),
        };
        if Weight::Inf == self.d_graph.get_weight(best_node) {
            return None;
        }
        return Some((best_node, d));
    }

    fn get_on_tree(&self, s: usize, l: usize) -> (usize, usize) {
        let internal_idx = self.d_graph.mapping[s].unwrap();
        let mut res = 0;
        match &self.d_graph.components[self.d_graph.comp_idx[s].unwrap()] {
            CompType::Ind(tree) => {
                let ancestor = tree.la.level_ancestor(internal_idx, l);
                let node = tree.beq.get(internal_idx, ancestor);
                res = tree.mapping[min(node.edge)];
            },
            CompType::Con(tree) => {
                let ancestor = tree.la.level_ancestor(internal_idx, l);
                let node = tree.beq.get(internal_idx, ancestor);
                res = tree.mapping[min(node.edge)];
            },
            CompType::Cyc(_) => panic!("get on trees called on cycle!"),
        }
        let d = self.d_graph.get_depth(s) - self.d_graph.get_depth(res);
        return (res, d);
    }

    // TODO check, if the indicies for lca are set right
    fn get_on_cycle(&self, s: usize, l: usize) -> (usize, usize) {
        let i = self.d_graph.mapping[s].unwrap();
        if let CompType::Cyc(cycle) = &self.d_graph.components[self.d_graph.comp_idx[s].unwrap()] {
            let len = cycle.nodes.len();
            let mut max_idx = 0;
            if l > 2*len {
                max_idx = cycle.lca.get(0, len - 1);
            }
            else {
                let j = (i + l) % len;
                max_idx = cycle.lca.get(i, len + j - 1);
            }
            //TODO get this cleaned plssss
            max_idx = max_idx % len;
            let d = (i + l) % len;
            if d <= max_idx {
                return (cycle.nodes[max_idx], l - d - (len - max_idx));
            }
            return (cycle.nodes[max_idx], l - d + max_idx);
        }
        else {
            panic!("get on cycle called on tree!");
        }
    }

    pub fn show(&self) {
        for comp in self.d_graph.components.iter() {
            match comp {
                CompType::Ind(tree) => println!("Ind{:?}", tree.mapping),
                CompType::Con(tree) => println!("Con{:?}", tree.mapping),
                CompType::Cyc(cycle) => println!("Cyc{:?}", cycle.nodes),
            };
        }
        for i in 0..self.d_graph.comp_idx.len(){
            let idx = self.d_graph.comp_idx[i].unwrap();
            match self.d_graph.components[idx] {
                CompType::Ind(_) => println!("{i}: Ind"),
                CompType::Con(_) => println!("{i}: Con"),
                CompType::Cyc(_) => println!("{i}: Cyc"),
            };
        }
    }
}
fn min(tuple: (usize, usize)) -> usize {
    if tuple.0 < tuple.1 {
        return tuple.0;
    }
    return tuple.1;
}
