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
    stack_s: Vec<(char,usize,usize)>,
}

impl Enumerate {
    pub fn new(delta: Vec<Vec<(char, usize)>>, n: usize) -> Self {
        let pmn = PathMaxNode::new(&delta);
        Self {
            delta: delta,
            pmn: pmn,
            n: n,
            stack_s: Vec::new(),
        }
    }

    pub fn recurse(&mut self, a: char, q: usize, l: usize, indent: usize, count: &mut usize, /*timing: &mut Instant*/) {
        // println!("called with a: {a}, q: {q}, l: {l}");
        self.stack_s.push((a,q,l)); //2
        //push stackframe of this call and top element of stack_s onto stack_c //3
        let ind: String = "\t".repeat(indent);
        //println!("elapsed time: {:?}", timing.elapsed());
        //*timing = Instant::now();
        // println!("{}Output: ({},{},{},{})",ind,self.n as i64 - l as i64 - 1,a,q,l);   //4
        *count += 1;
        if l == 0 {
            return; //5
        }
        let mut last: (char, usize, usize) = ('a',0,0);
        let mut u: VecDeque<((usize,usize),usize,usize)> = VecDeque::new(); //6
        if let Some((p,d)) = self.pmn.get(q,l) { //7
            // println!("PathMaxNode({q},{l}) = ({p},{d})");
            let (b, w, new_p) = self.pmn.d_graph.lq[p][1];  //10
            if Val(d as i64) + w >= Val(l as i64) {     //8
                u.push_back(((q, l), q, 0)); //9
                last = (b, new_p, l-d-1);   //11
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
            // println!("popped (({s}, {j}), {q}, {h})");
            let (next_q, f) = self.pmn.get(s,j).unwrap(); //17
            if let Some((r,e)) = self.pmn.get(s,f) {    // 18
                let w_r = self.pmn.d_graph.get_weight(r);
                // println!("h+e+1+w_r = {:?}",Val(h as i64) + Val(e as i64) + Val(1) + w_r);
                if f > 0 && Val(h as i64) + Val(e as i64) + Val(1) + w_r >= Val(l as i64) {
                    // println!("pushed (({s}, {f}), {q}, {h})+");
                    u.push_back(((s,f),q,h));
                }
            }
            let succ = self.pmn.d_graph.get_succesor(next_q).unwrap();
            if let Some((r,e)) = self.pmn.get(succ, j - f - 1) {   //19 fixed
                let w_r = self.pmn.d_graph.get_weight(r);
                if j - f - 1 > 0 && Val(h as i64)+Val(f as i64)+Val(1)+Val(e as i64)+Val(1)+w_r >= Val(l as i64) {
                    // println!("pushed (({succ}, {}), {q}, {})-",j - f - 1, h + f + 1);
                    u.push_back(((succ, j - f - 1), q, h + f + 1));
                }
            }
            let mut x = 2;  //22, 23        //third element
            if ((s,j),q,h) != ((q,l),q,0) { //20
                x = 1;  //21                //second element
            }
            while let Some((b, g, branch)) = self.pmn.d_graph.lq[next_q].get(x) {    //TODO change this to for each loop
                // println!("h+f+g+1 = {:?}", Val(h as i64) + Val(f as i64) + *g + Val(1));
                if Val(h as i64) + Val(f as i64) + *g + Val(1) < Val(l as i64) {
                    break;
                }
                self.recurse(*b, *branch, l - h - f - 1, indent + 1, count, /*timing*/);
                // set the top element of S to the element pointed by the top element of C
                x += 1;
            }
        }
        self.stack_s.pop();
        self.recurse(last.0, last.1, last.2, indent + 1, count, /*timing*/);
        return;
    }

    fn remove_this_call(&mut self) {
        self.stack_s.pop();
    }
}