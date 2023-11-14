mod lib;
use lib::patriotic_selections;

fn main() {
    let street = "RWGXXX";
    let result = patriotic_selections(street);
    println!("Number of patriotic selections: {}", result);
}
