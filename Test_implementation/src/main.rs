mod longest_path;

use std::collections::{HashSet, HashMap};
use longest_path::compute_longest_pairs;

fn main() {
    let mut v: Vec<HashMap<char, usize>> = Vec::new();
    let mut tmp = HashMap::new();
    tmp.insert('H', 1);
    v.push(tmp);
    tmp = HashMap::new();
    tmp.insert('e', 2);
    v.push(tmp);
    tmp = HashMap::new();
    tmp.insert('l', 3);
    v.push(tmp);
    tmp = HashMap::new();
    tmp.insert('l', 4);
    v.push(tmp);
    tmp = HashMap::new();
    tmp.insert('o', 5);
    v.push(tmp);
    v.push(HashMap::new());

    let res = compute_longest_pairs(&v);
    println!("{:?}", res);
}
