fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);
    println!("The values of tup are: ({}, {}, {})", tup.0, tup.1, tup.2);

    another_function();

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    println!("Another function.");
}