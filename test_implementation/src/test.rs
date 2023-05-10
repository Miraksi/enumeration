use std::collections::{HashSet, HashMap};

fn main() {
    let mut list: Vec<u32> = vec![2,3,4,5,6,7];
    list.reverse();
    println!("{:?}", list);
    let opt: Vec<Option<bool>> = vec![Option::None; 4];
    println!("{:?}", opt);
}