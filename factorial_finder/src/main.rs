use std::io;

fn factorial(number: u128) -> u128{
    match number{
        0 => 1,
        1 => 1,
        _ => factorial(number - 1) * number,
    }
}

fn main(){

    println!("Input a number:");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read from stdin");
    
    let number: u128 = num.trim().parse().unwrap();

    println!("You input {}.", number);

    let fact = factorial(number);
    println!("The factorial of {} is {},", number, fact)
}