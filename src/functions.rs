pub(crate) mod functions {
    use std::io;

    pub fn init() {
        let mut num1 = String::new();
        let mut num2 = String::new();

        println!("Enter Number 1: ");
        io::stdin().read_line(&mut num1).expect("Err");
        let num1: i32 = match num1.trim().parse() {
            Ok(nums) => nums,
            Err(_) => 0,
        };

        println!("Enter Number 2: ");
        io::stdin().read_line(&mut num2).expect("Err");
        let num2: i32 = match num2.trim().parse() {
            Ok(nums) => nums,
            Err(_) => 0,
        };

        let add: i32 = add_nums(num1, num2);
        let sub: i32 = sub_nums(num1, num2);
        let mul: i32 = mul_nums(num1, num2);
        let div: i32 = div_nums(num1, num2);

        println!("Addition is: {}", add);
        println!("Subtraction is: {}", sub);
        println!("Multipliction is: {}", mul);
        println!("Division is: {}", div);
    }

    // fn new_function(){
    //     println!("I am new Function");
    // }

    // fn int_function() -> i32{
    //     return 5;
    // }

    fn add_nums(x: i32, y: i32) -> i32 {
        return x + y;
    }

    fn sub_nums(x: i32, y: i32) -> i32 {
        return x - y;
    }

    fn mul_nums(x: i32, y: i32) -> i32 {
        return x * y;
    }

    fn div_nums(x: i32, y: i32) -> i32 {
        if y == 0 {
            println!("Sorry Cannot Divide by zero");
            return 0;
        }
        return x / y;
    }
}
