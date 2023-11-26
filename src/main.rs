fn main() {
    // immutable
    let x1 = 5;
    println!("the value of x1 is : {}", x1);
    // x1 = 6;
    // println!("the value of x1 is : {}", x1);

    // mutable
    let mut x2 = 15;
    println!("the value of x2 is : {}", x2);
    x2 = 16;
    println!("the value of x2 is : {}", x2);

    // shadowing
    let x3 = 5;
    let x3 = x3 + 1;
    {
        let x3 = x3 * 2;
        println!("The value of x3 in inner scope is: {}", x3);
    }
    println!("The value of x3 is :{}", x3);

    // mutable only for data not type
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // Floating points
    let f1 = 2.0; // f64
    let f2: f32 = 3.0; // f32

    println!("The value of f1 is : {f1:0.3}");
    println!("The value of f1 is : {f2:0.3}");

    // Numeric Operations

    // addition
    let sum = 5 + 10;
    println!("Sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("Product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("Quotient: {}", quotient);

    // demonstrating integer division (truncation towards zero)
    let truncated = -5 / 3; // Results in -1 since it's integer division
    println!("Truncated division result: {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);
}
