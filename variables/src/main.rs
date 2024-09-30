fn main() {
    let mut x = 5;
    println!("x value {x}");

    x = 6;
    println!("x value {x}");

    shadow();
    num_operations();
    compound_type();
    array();
    parameter(5);
    expression();
    let x = plus_one(5);
    println!("The value of plus one is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn expression(){
    let y = {
        let x = 1;
        x+1
    };
    println!("The value of yyyyy is: {y}");
}

fn parameter(x:i32){
        println!("x parameter value is {x}");
}

fn array(){
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
}


fn shadow(){
    let y = 5;

    let y = 6;

    {
        let y = y*2;
        println!("y value is {y}");
    }
    println!("y value is {y}");


    let spaces = "   ";
    let spaces = spaces.len();
    
    println!("space content {spaces}");

    //let mut spaces = "   ";
    //spaces = spaces.len(); compile error because we can't change type from a mutable variable
}

fn num_operations(){

     // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

fn compound_type(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");


    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
}
