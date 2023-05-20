use std::collections::{HashSet, HashMap};

fn main() {
    let (arr1, arr2) = foo(vec![32], vec![23, 32]);
    println!("{:?}, {:?}", arr1, arr2);
}

fn foo(mut arr1: Vec<usize>, mut arr2: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    arr1.push(arr2.len());
    arr2.push(arr1.len());
    return (arr1, arr2);
}