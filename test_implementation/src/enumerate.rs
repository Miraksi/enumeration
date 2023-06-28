pub mod path_max_node;

use path_max_node::PathMaxNode;
use std::collections::HashMap;

pub struct Enumerate {
    pmn: PathMaxNode,
    stack_s: Vec<(char,usize,usize)>,
}

impl Enumerate {
    pub fn new(delta: &Vec<HashMap<char, usize>>) -> Self {
        Self {
            pmn: PathMaxNode::new(delta),
            stack_s: Vec::new(),
        }
    }

    pub fn recurse(&mut self, a: char, state: usize, l: usize) {
        self.stack_s.push((a,state,l));
        //push stackframe of this call and top element of stack_s onto stack_c 
        //Output(n-l-1,a,q,l)
        if l == 0 {
            return;
        }

        let mut u: Vec<((usize,usize),usize,usize)> = Vec::new();
    }
}