fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, use methods that maintain memory safety
    v[0] = 10; 
    println!("{:?}", v);
}