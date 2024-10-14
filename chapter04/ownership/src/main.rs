fn main() {
    let immut  = "aaa";
    println!("{immut}");
   
    //{ adding these brackets, you will turn s variable out of scope to println
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    //}
    println!("{s}"); // This will print `hello, world!`


    //invalidate s1 to avoid a double free error
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");


}
