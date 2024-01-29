# Enumeration
This repo implements the algorithm described in the paper 'Enumerating Prefix-Closed Regular Languages with Constant Delay' (see Resources) and therefore also implements all standard data-structures needed for the algorithm.
I've also tried to get as close to the theoretical construction and complexity as possible.

## Implemented  Data-Structures
- Range Minimum Query (O(n), O(1))
- Sparse Table
- Lowest Common Ancestor (O(n), O(1))
- Level Ancestor Query (O(n), O(1))
- Topological Sorting
- Tarjan's Algorithm for strongly connected components
- Construction of Cartesian trees on Arrays and edge-weighed Trees
- Decremental Connectivity (O(n), O(1))
- Even and Shiloach’s algorithm for decremental connectivity

## Documentation
To get the html just run 
```
cargo doc
```

## Benchmarks
Code to benchmark this project can be executed by running
```
cargo bench
```