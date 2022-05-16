use std::io;

fn main() {
    println!("Enter first number");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read number");
    num1 = num1.parse().unwrap();

    println!("Enter second number");

    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read number");
    num2 = num2.parse().unwrap(); println!("First number: {} and Second number: {}",num1,num2);

    println!("Press 1 to add and 2 to substract");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read the operator");
    op = op.parse().unwrap();
    println!("your choice is {}", op);

}

fn sum(i: i32, j: i32)-> i32{
    i+j
}

fn substract(i: i32, j: i32)-> i32{
    i-j
}
