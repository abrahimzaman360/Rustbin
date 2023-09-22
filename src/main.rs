use std::process::exit;

mod functions;
mod guess;
mod tictactoe;

fn main() {
    println!("Which Program You Want to Run");
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Error 10k!");
    if inp.contains("1") {
        guess::game::init();
    }
    else if inp.contains("2") {
        tictactoe::tictactoe::init();
    }
    else if inp.contains("3") {
        functions::functions::init();
    }
    exit(200);
}
