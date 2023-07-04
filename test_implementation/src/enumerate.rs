pub mod path_max_node;

use path_max_node::PathMaxNode;
use std::collections::HashMap;
use crate::weight::Weight::*;
use std::collections::VecDeque;

pub struct Enumerate {
    delta: Vec<HashMap<char, usize>>,
    pub pmn: PathMaxNode,
    n: usize,
    stack_s: Vec<(char,usize,usize)>,
}

impl Enumerate {
    pub fn new(delta: Vec<HashMap<char, usize>>) -> Self {
        let pmn = PathMaxNode::new(&delta);
        Self {
            delta: delta,
            pmn: pmn,
            n: 0,
            stack_s: Vec::new(),
        }
    }
    pub fn set_n(&mut self, n: usize) {
        self.n = n;
    }

    pub fn recurse(&mut self, a: char, q: usize, l: usize) {
        println!("called with a: {a}, q: {q}, l: {l}");
        self.stack_s.push((a,q,l)); //2
        //push stackframe of this call and top element of stack_s onto stack_c //3
        println!("Output: ({},{},{},{})",self.n as i64 - l as i64 - 1,a,q,l);   //4
        if l == 0 {
            return; //5
        }
        let mut last: (char, usize, usize) = ('a',0,0);
        let mut u: VecDeque<((usize,usize),usize,usize)> = VecDeque::new(); //6
        if let Some((p,d)) = self.pmn.get(q,l) { //7
            println!("PathMaxNode({q},{l}) = ({p},{d})");
            let (b,w) = self.pmn.d_graph.lq[p][1];  //10
            println!("w_{p}: {:?}", w);
            if Val(d as i64) + w >= Val(l as i64) {     //8
                u.push_back(((q,l),q,0)); //9
                let new_p = self.delta[p].get(&b).unwrap();
                last = (b, *new_p, l-d-1);   //11
            }
            else {  //12
                self.remove_this_call();    //13
                return; //14
            }
        }
        else {  //12
            self.remove_this_call();  //13
            return;//14
        }
        while !u.is_empty() {   //16
            let ((s,j),q,h) = u.pop_front().unwrap();   //17
            let (next_q, f) = self.pmn.get(s,j).unwrap(); //17
            println!("PathMaxNode({s},{j}) = ({next_q},{f})");
            match self.pmn.get(s,f) {   //18
                Some((r,e)) => {
                    let w_r = self.pmn.d_graph.get_weight(r);
                    if f > 0 && Val(h as i64) + Val(e as i64) + Val(1) + w_r >= Val(l as i64) {
                        u.push_back(((s,f),q,h));
                    }
                },
                None => continue,
            };
            let succ = self.pmn.d_graph.get_succesor(next_q).unwrap();
            match self.pmn.get(succ, f) {   //19
                Some((r,e)) => {
                    let w_r = self.pmn.d_graph.get_weight(r);
                    if j - f - 1 > 0 && Val(h as i64)+Val(f as i64)+Val(1)+Val(e as i64)+Val(1)+w_r >= Val(l as i64) {
                        u.push_back(((succ, j - f - 1), q, h + f + 1));
                    }
                },
                None => continue,
            };
            let mut x = 2;  //22, 23        //third element
            if ((s,f),q,h) != ((q,l),q,0) { //20
                x = 1;  //21                //second element
            }
            while let Some((b,g)) = self.pmn.d_graph.lq[next_q].get(x) {    //TODO change this to for each loop
                if Val(h as i64) + Val(f as i64) + *g + Val(1) < Val(l as i64) {
                    break;
                }
                println!("recurse");
                let branch = self.delta[next_q].get(b).unwrap();
                self.recurse(*b, *branch, l - h - f - 1);
                // set the top element of S to the element pointed by the top element of C
                x += 1;
            }
        }
        println!("last");
        self.stack_s.pop();
        self.recurse(last.0, last.1, last.2);
        return;
    }

    fn remove_this_call(&mut self) {
        self.stack_s.pop();
    }
}