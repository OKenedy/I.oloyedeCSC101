// Rust program to calculate the roots of a quadratic equation

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first value: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter second value: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter third value: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = (b * b) - 4.0 * a *c;
     if d > 0.0 {
        println!("Two distinct roots");
    }
    if d < 0.0 {
        println!("No real root");
    }
    if d == 0.0 {
        println!("exactly one real root");
    }
}
