use holiday_planning::plan;
use std::io::{self, BufRead};

fn main() {

    // Read n and D from the first line
    let mut input_line = String::new();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let mut iter = input_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _d: usize = iter.next().unwrap().parse().unwrap();

    // Initialize a 2D vector to store the values
    let mut attractions: Vec<Vec<usize>> = Vec::with_capacity(n);

    // Process each of the following n lines
    for _ in 0..n {
        input_line.clear();
        io::stdin().lock().read_line(&mut input_line).unwrap();
        let row: Vec<usize> = input_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Save the row to the 2D vector
        attractions.push(row);
    }

    let result = plan(attractions);
    println!("{}", result);
}
