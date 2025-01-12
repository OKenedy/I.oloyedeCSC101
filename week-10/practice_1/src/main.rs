fn main() {
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    // Only a single variable owns the heap memory at any given time
    let v2 = v;
    // Here two variables own the heap value,
    // Two pointers to the same content is not allowed in Rust

    // Rust is very smart in terms of memory access, so it detects a race condition
    // as two variables point to the same heap

    println!("{:?}", v2);
}
