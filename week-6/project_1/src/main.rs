use std::io;

fn area_of_trapezium(h: i32,b1: i32,b2: i32) {
    let Area = h / 2 * (b1 + b2);

    let mut input1 = String::new();
    println!("Enter input for height:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let h:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for base 1:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b1:i32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter input for base 2:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let b2:i32 = input3.trim().parse().expect("Invalid input");

    Println!("Area of trapezium = {}", area);
}

fn area_of_rhombus(d1: i32,d2: i32) {
    let Area = 0.5 * d1 * d2;

    let mut input1 = String::new();
    println!("Enter input for diagonal 1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let d1:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for diagonal 2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let d2:i32 = input2.trim().parse().expect("Invalid input");

    Println!("Area of rhombus = {}", area);
}

fn area_of_parallelogram(b: i32,a: i32) {
    let Area = b * a;

    let mut input1 = String::new();
    println!("Enter input for base:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let b:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for altitude:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let a:i32 = input2.trim().parse().expect("Invalid input");

    Println!("Area of parallelogram = {}", area);
}

fn area_of_cube(l: i32) {
    let Area = 6 * l.powf(2).0
    ;

    let mut input1 = String::new();
    println!("Enter input for length:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let l:i32 = input1.trim().parse().expect("Invalid input");

    Println!("Area of cube = {}", area);
}

fn area_of_cylinder(r: i32,h: i32) {
    let Area = (22 / 7) * r.powf(2) * h ;

    let mut input1 = String::new();
    println!("Enter input for height:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let h:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for radius:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let r:i32 = input2.trim().parse().expect("Invalid input");

    Println!("Area of cylinder = {}", area);
}
fn main() {
    println!("what equation do you want to use:");
    
    if
    //call area
    area_of_trapezium(h, b1, b2);
}