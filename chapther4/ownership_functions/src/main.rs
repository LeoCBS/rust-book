/*fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    //println!("{s}");

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
*/
/*
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
*/

//ref and borrow example
/*fn main(){
let s1 = String::from("leo");

let len = calculate_len(&s1);

    println!("the len of {s1} is len {len}");
}

fn calculate_len(s :&String) -> usize{
    s.len()
}*/

//mutable ref
/*fn main(){
    let mut name = String::from("leonardo");
    change(&mut name);
    println!("{name}");
}

fn change(name: &mut String){
    name.push_str(" borges")
}*/

//dangling ref
/*fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("leonardo");
    &s // error because variable s will out of scope, and will be droped
}*/
