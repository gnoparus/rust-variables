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

    let f1 = 2.0; // f64
    let f2: f32 = 3.0; // f32

    println!("The value of f1 is : {f1:0.3}");
    println!("The value of f1 is : {f2:0.3}");
}
