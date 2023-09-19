use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    println!("Enter X: ");
    io::stdin().read_line(&mut x).expect("ERR");
    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Enter Y: ");
    io::stdin().read_line(&mut y).expect("ERR");
    let y: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if x > y {
        println!("X is Larger Then Y");
    } else if x < y {
        println!("Y is Larger Then X");
    } else {
        println!("Both are equal");
    }
}
