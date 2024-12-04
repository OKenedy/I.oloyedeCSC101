fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let vec3: Vec<_> = vec1.iter().chain(vec2.iter()).cloned().collect();
    println!("Combined Vector: {:?}", vec3);
}
