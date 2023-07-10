extern crate rdxsort;
use rdxsort::RdxSort;

use std::ops::{Add,Neg,Sub};
use std::cmp::{PartialOrd,Ordering};
use Weight::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Data-type to handle lengths of longest pahts along a graph. Since this can be infinit, Inf and NInf have to be added. 
pub enum Weight {
    Val(i64),
    Inf,
    NInf,
}
impl Add for Weight {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Val(x), Val(y)) => Weight::Val(x+y),
            (Inf, Val(_)) => Weight::Inf,
            (Val(_), Inf) => Weight::Inf,
            (Inf, Inf) => Weight::Inf,
            (_,_) => Weight::NInf,
        }
    }
}
impl Sub for Weight {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return self + (-other);
    }
}
impl Neg for Weight {
    type Output = Self;
    fn neg(self) -> Self {
        match self{
            Val(x) => Val(-x),
            Inf => NInf,
            NInf => Inf,
        }
    }
}
impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Val(x),Val(y)) => Some(x.cmp(y)),
            (NInf, NInf) => Some(Ordering::Equal),
            (NInf, _) => Some(Ordering::Less),
            (_, NInf) => Some(Ordering::Greater),
            (Inf,Inf) => Some(Ordering::Equal),
            (_,Inf) => Some(Ordering::Less),
            (Inf, _) => Some(Ordering::Greater),
        }
    }
}
impl Ord for Weight {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Val(x),Val(y)) => x.cmp(y),
            (NInf, NInf) => Ordering::Equal,
            (NInf, _) => Ordering::Less,
            (_, NInf) => Ordering::Greater,
            (Inf,Inf) => Ordering::Equal,
            (_,Inf) => Ordering::Less,
            (Inf, _) => Ordering::Greater,
        }
    }
}

pub fn w_rdxsort(inp: Vec<(Weight,(usize, usize))>) -> Vec<(Weight,(usize, usize))> {
    let mut arr: Vec<(i64,usize,usize)> = Vec::new();
    for w in inp.iter() {
        match *w {
            (Val(x),(y,z)) => arr.push((x,y,z)),
            (Inf,(y,z)) => arr.push((i64::MAX,y,z)),
            (NInf,(y,z)) => arr.push((i64::MIN,y,z)),
        }
    }
    arr.rdxsort();
    let mut new: Vec<(Weight,(usize,usize))> = Vec::new();
    for w in arr.iter() {
        match *w {
            (i64::MAX,y,z) => new.push((Inf,(y,z))),
            (i64::MIN,y,z) => new.push((Inf,(y,z))),
            (x,y,z) => new.push((Val(x),(y,z))),
        };
    }
    return new;
}