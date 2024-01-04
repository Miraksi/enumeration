pub mod path_max_node;

use path_max_node::PathMaxNode;
use crate::weight::Weight::*;
use std::collections::VecDeque;

use std::time::Instant;

/// The main data-structure for the enumeration of prefix closed regular languages,
/// given by the delta of an atomaton accepting it.
///
/// # Complexity 
/// preprocessing runs in linear time, and the enumeration all words of size n
/// runs also in linear time with constant delay.
///
/// # Sources 
/// Main algorithm of 'D. Adamson, F. Manea and P. Gawrychowski. Enumerating Prefix-Closed Regular Languages with Constant Delay'
pub struct Enumerate {
    delta: Vec<Vec<(char, usize)>>,
    pub pmn: PathMaxNode,
    n: usize,
}

impl Enumerate {
    pub fn new(delta: Vec<Vec<(char, usize)>>, n: usize) -> Self {
        let pmn = PathMaxNode::new(&delta);
        Self {
            delta: delta,
            pmn: pmn,
            n: n,
        }
    }

    pub fn recurse(&self, a: char, q: usize, l: usize, indent: usize, stack_s: &mut Vec<(char,usize,usize)>, count: &mut usize, /*timing: &mut Instant*/) {
        stack_s.push((a,q,l)); //2
        // push stackframe of this call and top element of stack_s onto stack_c //3
        //println!("elapsed time: {:?}", timing.elapsed());
        //*timing = Instant::now();
        // let ind: String = "\t".repeat(indent);
        // println!("{}Output: ({},{},{},{})",ind,self.n as i64 - l as i64 - 1,a,q,l);   //4
        *count += 1;
        if l == 0 {
            return; //5
        }
        // let mut last: (char, usize, usize) = ('a',0,0);
        let mut u: VecDeque<((usize,usize),usize)> = VecDeque::new(); //6
        if let Some((p,d)) = self.pmn.get(q,l) { //7
            let (b, w, new_p) = self.pmn.d_graph.lq[p][1];  //10
            if Val(d as i64) + w >= Val(l as i64) {     //8
                u.push_back(((q, l), 0)); //9
                // last = (b, new_p, l-d-1);   //11
            }
            else {  //12
                self.remove_this_call(stack_s);    //13
                return; //14
            }
        }
        else {  //12
            self.remove_this_call(stack_s);  //13
            return;//14
        }
        while !u.is_empty() {   //16
            let ((s,j),h) = u.pop_front().unwrap();   //17
            let (next_q, f) = self.pmn.get(s,j).unwrap(); //17
            if let Some((r,e)) = self.pmn.get(s,f) {    // 18
                let w_r = self.pmn.d_graph.get_weight(r);
                if f > 0 && Val(h as i64) + Val(e as i64) + Val(1) + w_r >= Val(l as i64) {
                    u.push_back(((s,f),h));
                }
            }
            let succ = self.pmn.d_graph.get_succesor(next_q).unwrap();
            if let Some((r,e)) = self.pmn.get(succ, j - f - 1) {   //19 fixed
                let w_r = self.pmn.d_graph.get_weight(r);
                if j - f - 1 > 0 && Val(h as i64)+Val(f as i64)+Val(1)+Val(e as i64)+Val(1)+w_r >= Val(l as i64) {
                    u.push_back(((succ, j - f - 1), h + f + 1));
                }
            }
            let mut x = 1;
            let lq_ref = &self.pmn.d_graph.lq[next_q];
            for (b, g, branch) in lq_ref[x..].iter() {
                if Val(h as i64) + Val(f as i64) + *g + Val(1) < Val(l as i64) {
                    break;
                }
                self.recurse(*b, *branch, l - h - f - 1, indent + 1, stack_s, count, /*timing*/);
                // set the top element of S to the element pointed by the top element of C
            }
        }
        stack_s.pop();
        return;
    }

    fn remove_this_call(&self, stack_s: &mut Vec<(char,usize,usize)>) {
        stack_s.pop();
    }
}