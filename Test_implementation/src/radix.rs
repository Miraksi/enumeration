use rdxsort::*;

fn radix(m: u32, list: &Vec<u32>) -> Vec<u32> {
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); m as usize];

    for x in list {
        let idx: usize = (*x%m) as usize;
        buckets[idx].push(*x);
    }
    for lst in buckets.iter_mut() {
        lst.sort();
    }
    return merge(&buckets);
}

fn merge(buckets: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut sorted: Vec<u32> = Vec::new();
    for bucket in buckets.iter() {
        for x in bucket.iter() {
            sorted.push(*x);
        }
    }
    return sorted;
}

fn main() {
    let list: Vec<u32> = vec![123,312,346,765,234,543,567,897,234];
    println!("{:?}", radix(10, &list));
}