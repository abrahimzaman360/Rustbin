use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // From Documentation. Lets Try Guess The Number Game
    println!("This is Guess the number Game");

    let number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter Your Guess: ");
        println!("The Number is: {number}");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&number) {
            Ordering::Greater => {
                println!("Big")
            }
            Ordering::Less => {
                println!("Small")
            }
            Ordering::Equal => {
                println!("You Guessed it!!");
                break;
            }
        }
    }
}
