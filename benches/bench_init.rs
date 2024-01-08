use std::collections::HashMap;
use test_implementation::enumerate::Enumerate;
use criterion::*;
use std::time::Duration;

fn benchmark_init(c: &mut Criterion) {
    let mut delta: Vec<Vec<(char, usize)>> = Vec::new();

    delta.push(vec![('a', 1), ('b', 4)]);
    delta.push(vec![('a', 2)]);
    delta.push(vec![('b', 1), ('a', 3)]);
    delta.push(vec![('a', 0)]);
    delta.push(vec![('b', 5),('a', 9)]);
    delta.push(vec![('b', 6),('a', 10)]);
    delta.push(vec![('a', 7),('b', 3)]);
    delta.push(vec![('a', 8)]);
    delta.push(vec![('b', 8)]);
    delta.push(vec![('a', 11),('b', 12)]);
    delta.push(vec![('a', 13),('b', 14)]);
    delta.push(vec![('a', 15),('b', 16)]);
    delta.push(vec![('a', 17),('b', 18)]);
    delta.push(vec![('a', 19),('b', 20)]);
    delta.push(vec![('a', 21),('b', 22)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![('a', 23)]);
    delta.push(vec![]);
    let enumerate = Enumerate::new(delta.clone());

    let mut group = c.benchmark_group("benchmark_init");
    for x in (10..=45).step_by(5) {
        let n: usize = x;
        let mut count = 0;
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| b.iter(|| {
            count = enumerate.start_enumeration(n);
        }));
        println!("count: {count}");
    }
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().measurement_time(Duration::new(30,0));
    targets = benchmark_init
}
criterion_main!(benches);