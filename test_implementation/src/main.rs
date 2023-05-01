mod level_ancestor;

use level_ancestor::Ladders;

fn main() {
    let parent: Vec<usize> = vec![0,0,1,0,3,3,2];
    let children: Vec<Vec<usize>> = vec![vec![1,3], vec![2], vec![6], vec![4,5], Vec::new(), Vec::new(), Vec::new()];
    let lad = Ladders::new(parent, children, 0);
    println!("{:?}", lad.ladders);
}
