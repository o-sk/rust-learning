fn main() {
    let s = String::from("hello");

    let len = calculate_length(&s);
    println!("length: {}", len);

    takes_ownership(s);
    // takes_ownership(s); // Error: use of moved value

    let x = 5;

    makes_copy(x);
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
