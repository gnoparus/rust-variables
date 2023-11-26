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

    // Boolean type

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    // Character Type

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // Tuple

    // Tuple with explicit type annotations.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Tuple with inferred types.
    let tup2 = (500, 6.4, 1);

    // Destructure the first tuple.
    let (a, b, c) = tup;

    // Destructure the second tuple.
    let (x, y, z) = tup2;

    // Print the values of the destructured variables from the first tuple.
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);

    // Print the values of the destructured variables from the second tuple.
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // Define tuple with explicit type annotations and assign it to 'tup3'
    let tup3: (i32, f64, u8) = (500, 6.4, 1);

    // Access elements of 'tup3' by their index and assign to variables
    let five_hundred = tup3.0;
    let six_point_four = tup3.1;
    let one = tup3.2;

    // Print the values of the variables
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // Array
    let a1 = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    let a3 = [3; 5];

    // Accessing elements from array 'a1' and print
    println!("The first element of a1 is: {}", a1[0]);
    println!("The second element of a1 is: {}", a1[1]);

    // Iterating over the 'months' array and print each value
    for month in &months {
        println!("Month: {}", month);
    }

    // Display the entire 'a2' array
    println!("Array a2 contains: {:?}", a2);

    // Print length of array 'a3'
    println!("Array a3 has {} elements: {:?}", a3.len(), a3);
}
