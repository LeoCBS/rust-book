fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

   //s.clear(); // this empties the String, making it equal to ""
    
    println!("{word}");

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
