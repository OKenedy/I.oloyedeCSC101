// Rust program to calculate the area of a triangle given three sides

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter experience level: {}",experienced);
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter experience level: {}",inexperienced);
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid number");

    if input1 == experienced && age >= 40.0
    {
        println!("employees incentive: {}",1560000);
    }
        if input1 == experienced && age <= 40.0 && age >= 30.0
    {
        println!("employees incentive: {}",1480000);
    }
        if input1 == experienced && age <= 28.0
    {
        println!("employees incentive: {} per month",1300000);
    }
    if input1 == inexperienced
    {
        println!("employees incentive: {}",100000);
    }
}
