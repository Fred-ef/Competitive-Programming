use holiday_planning::plan;

fn main() {
    // Example usage
    let attractions = vec![
        vec![3, 1, 1],
        vec![3, 2, 1],
        // vec![4, 6, 5, 7],
    ];

    let result = plan(attractions);
    println!("Maximum attractions visited: {}", result);
}