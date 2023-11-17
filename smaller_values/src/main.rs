mod segtree;
use segtree::SegTree;
use std::io::{self, BufRead};

fn main() {
    // Read n and m from the first line
    let mut input_line = String::new();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let mut iter = input_line.split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Read n integers from the second line into an array
    input_line.clear();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let arr: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read m lines with i, j, and X into an array of triples
    let mut queries: Vec<(usize, usize, i32)> = Vec::new();
    for _ in 0..m {
        input_line.clear();
        io::stdin().lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let i: usize = iter.next().unwrap().parse().unwrap();
        let j: usize = iter.next().unwrap().parse().unwrap();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        queries.push((i, j, x));
    }

    // building the segment tree
    let tree = SegTree::from_array(arr);

    // computing the queries and printing their results
    for query in queries {
        println!("{}", tree.smaller_values(query));
    }
}
