use std::collections::{HashSet, HashMap};
use std::ops::{Add,Neg,Sub};
use std::cmp::{PartialOrd,Ordering};
use Weight::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Weight {
    Val(i64),
    Inf,
    NInf,
}
impl Add for Weight {
    type Output = Self;
    //TODO check if this is a fitting addidtion
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
    //TODO check if this is a fitting addidtion
    fn sub(self, other: Self) -> Self {
        return self + (-other);
    }
}
impl Neg for Weight {
    type Output = Self;
    //TODO check if this is a fitting addidtion
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



fn main() {
    let x = Val(-1);
    let y = Inf;
    println!("{:?}",x-y);
    let mut arr = vec![Val(-1), Inf, NInf, Val(3), Val(5), Inf, NInf];
    arr.sort();
    println!("{:?}", arr);
    println!("{:?}", Val(-2) == Val(-2));
}
