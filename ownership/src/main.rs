fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
