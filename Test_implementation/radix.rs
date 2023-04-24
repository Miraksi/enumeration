fn radix(m: u32, list: &Vec<u32>) {
    let mut buckets: Vec<Vec<(u32,u32)>> = vec![Vec::new(); m as usize];

    for x in list {
        let idx: usize = (*x%m) as usize;
        buckets[idx].push((*x, *x/m));
    }
    for lst in buckets.iter_mut() {
        lst.sort();
    }
    println!("{:?}", buckets);
}

fn main() {
    let list: Vec<u32> = vec![123,312,346,765,234,543,567,897,234];
    radix(10, &list);
}