use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    loop {
        let secret = rand::thread_rng().gen_range(1..=100);
        println!("Guess the number");

        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need to enter a numerical value");
                continue;
            }
        };

        println!("Your guess was: {guess}\nThe answer is: {secret}\n");

        match guess.cmp(&secret) {
            Ordering::Less | Ordering::Greater => println!("You lost!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
