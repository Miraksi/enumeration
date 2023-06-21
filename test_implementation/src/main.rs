mod beq;
mod default_graph;
mod path_max_node;
mod level_ancestor;

use beq::cartesian::cartesian_on_list;


fn main() {
    let list: Vec<i64> = vec![6,9,2,4,7,8,5,8,3,7];
    println!("{:?}", cartesian_on_list(&list));
}
