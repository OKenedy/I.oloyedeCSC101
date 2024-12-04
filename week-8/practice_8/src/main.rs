fn print_tuple(data: (i32, f64, &str)) {
    println!("Tuple elements: {:?}", data);
}

fn main() {
    let tuple = (10, 20.5, "Rust");
    print_tuple(tuple);
}
