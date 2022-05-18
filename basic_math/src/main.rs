use std::io;

fn main() {
    println!("Enter first number");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read number");
    let a: i32 = num1.trim().parse().unwrap();

    println!("Enter second number");

    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read number");
    let b: i32 = num2.trim().parse().unwrap();
    println!("First number: {} and Second number: {}",num1,num2);

    println!("Press 1 to add, 2 to substract, 3 to multiply and 4 for division");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read the operator");
    let trimmed = op.trim().parse::<i32>().unwrap();

   match trimmed{
       1 => {
           let k = sum(a,b);
           println!("Result is {}", k);
       },
       2 => {
           let k = substract(a,b);
           println!("Result is {}", k);
       },
      3 => {
          let k = mul(a,b);
          println!("Result is {}", k);
      },
      4 => {
          let k = div(a,b);
          if k==0{
              println!("Cannot divisible by 0");
          }else{
          println!("Result is {}", k);
          }
      },
       _ => {
           println!("Wrong operator");
       }
   }
}
fn sum(i: i32, j: i32)->i32 {
    println!("The value of x is: {} {}", i,j);
    return i+j;
}

fn substract(i: i32, j: i32)->i32{
    return i-j;
}

fn div(i: i32, j: i32)->i32 {
    if j>0 {
       return i/j;
    }else{
        return 0;
    }
}

fn mul(i: i32, j: i32)->i32 {
    return i*j;
}
