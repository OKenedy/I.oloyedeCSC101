fn main() {
    let vec = vec![10, 20, 30, 40];
    if let Some(value) = vec.get(2) {
        println!("Element at index 2: {}", value);
    } else {
        println!("No element found!");
    }
}
