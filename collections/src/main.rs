fn main() {
    println!("Hello, world!");

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v: {:?}", v);
}
