use std::fs::File;
use std::io::{self, Read};

//https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
fn main() {
    println!("Hello, world!");
    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");
    let file_result = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            let kind = error.kind();
            match kind {
                std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}");
                }
            }
        }
    };

    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// also works with Option enum
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last() //we can use .ok_or to convert to result
}
