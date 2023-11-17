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

    // Read n integers from the second line into a vector
    input_line.clear();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let arr: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read queries and store them in the proper array
    let mut update_queries: Vec<(usize, usize, i32)> = Vec::new();
    let mut max_queries: Vec<(usize, usize)> = Vec::new();
    for _ in 0..m {
        input_line.clear();
        io::stdin().lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();

        // get left and right indeces
        let query_type: usize = iter.next().unwrap().parse().unwrap();
        let i: usize = iter.next().unwrap().parse().unwrap();
        let j: usize = iter.next().unwrap().parse().unwrap();

        if query_type == 0 {
            // store Update query with i, j, and t
            let t: i32 = iter.next().unwrap().parse().unwrap();
            update_queries.push((i-1, j-1, t));
        } else {
            // store Max query with i and j
            max_queries.push((i-1, j-1));
        }
    }

    // building the segment tree
    let mut tree = SegTree::from_array(arr);

    // computing the updates
    for query in update_queries {
        tree.update(query);
    }

    // computing the max queries and printing their results
    for query in max_queries {
        println!("{}", tree.max(query));
    }
}
