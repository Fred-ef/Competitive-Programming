use queries_of_operations::queries_of_operations;
use std::io::{self, BufRead};

fn main() {// Read n, m, and k from the first line
    let mut input_line = String::new();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let mut iter = input_line.split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Read n integers from the second line into a vector
    input_line.clear();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let mut arr: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Initialize vectors to store operations and queries
    let mut operations: Vec<(usize, usize, i32)> = Vec::with_capacity(m);
    let mut queries: Vec<(usize, usize)> = Vec::with_capacity(k);

    // store operations
    for _ in 0..m {
        input_line.clear();
        io::stdin().lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let d: i32 = iter.next().unwrap().parse().unwrap();

        // Store operation into the vector
        operations.push((l-1, r-1, d));
    }

    // store queries
    for _ in 0..k {
        input_line.clear();
        io::stdin().lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        // Store query into the vector
        queries.push((l-1, r-1));
    }

    // performing all operations
    queries_of_operations(&mut arr, operations, queries);

    // printing the results
    for el in arr {
        print!("{} ", el);
    }
}