use rdxsort::*;

fn main() {
    let mut list = vec![123,541234,543,123,12,345,654];
    list.rdxsort();
    println!("{:?}",list)
}
