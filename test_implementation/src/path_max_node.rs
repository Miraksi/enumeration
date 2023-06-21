use std::collections::HashMap;
use crate::default_graph::{DefaultGraph, CompType};

pub struct PathMaxNode {
    d_graph: DefaultGraph,
}

impl PathMaxNode {
    pub fn new(delta: &Vec<HashMap<char, usize>>) -> Self {
        Self {
            d_graph: DefaultGraph::new(delta),
        }
    }

    pub fn get(&self, s: usize, l: usize) -> usize {
        match &self.d_graph.components[self.d_graph.comp_idx[s].unwrap()] {
            CompType::Ind(tree) => return self.get_on_tree(s,l),
            CompType::Con(tree) => {
                let depth = self.d_graph.get_depth(s);
                if  depth > l {
                    return self.get_on_tree(s, l);
                }
                else {
                    return self.get_on_cycle(tree.mapping[0], l - depth);
                }
            },
            CompType::Cyc(cycle) => return self.get_on_cycle(s, l),
        }
    }

    fn get_on_tree(&self, s: usize, l: usize) -> usize {
        todo!();
    }

    fn get_on_cycle(&self, s: usize, l: usize) -> usize {
        todo!();
    }
}
