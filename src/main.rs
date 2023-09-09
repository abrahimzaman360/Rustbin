fn main() {
    let s = String::from("hello");
    let mut step: i32 = 0;
    println!("I have following length: {}", s.len());
    println!("How much storage capacity I have: {}", s.capacity());
    for i in s.chars() {
        println!("{:?} - {}", i, step);
        step += 1;
    }
}
