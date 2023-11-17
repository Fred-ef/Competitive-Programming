use xmas_lights::patriotic_selections;
use std::io::{self, BufRead};

fn main() {

    // Read the number of houses (n) from the first line
    let mut input_line = String::new();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let _n: usize = input_line.trim().parse().unwrap();

    // Read the colors of the n houses from the second line
    input_line.clear();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let colors: Vec<char> = input_line.trim().chars().collect();


    let result = patriotic_selections(colors);
    println!("{}", result);
}
