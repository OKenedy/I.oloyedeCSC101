fn main() {
    let mut arr = [10, 20, 30, 40];
    let slice = &mut arr[1..3];
    slice[0] = 50;
    println!("Modified Array: {:?}", arr);
}
