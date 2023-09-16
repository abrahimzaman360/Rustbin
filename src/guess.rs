pub(crate) mod game {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    pub fn init() {
        // From Documentation. Lets Try Guess The Number Game
        println!("\nThis is Guess the number Game");

        let number = rand::thread_rng().gen_range(1..=100);

        println!("The Number is: {number}");
        loop {
            println!("Enter Your Guess: ");
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
                    println!("Wrong guess, You gone higher the target!")
                }
                Ordering::Less => {
                    println!("Wrong guess, You gone lower the target!")
                }
                Ordering::Equal => {
                    println!("Wow, You Guessed it right!! The Number is: {number}");
                    println!("Do you want to play again?");
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).expect("E to play again & Q/q to quit");
                    if buf.trim() == "Q" || buf.trim() == "q" {
                        break;
                    } else if buf.trim() == "Y" || buf.trim() == "y" {
                        init();
                    }
                    break;
                }
            }
        }
    }
}
