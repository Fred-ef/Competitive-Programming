mod lib;
use lib::max_attractions;

fn main() {
    let cities = 3;
    let days = 4;
    let attractions = vec![
        vec![3, 2, 1, 4],
        vec![4, 3, 2, 3],
        vec![2, 2, 2, 3],
    ];

    let max = max_attractions(&attractions, cities, days);
    println!("Maximum attractions: {}", max);
}