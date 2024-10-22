enum SpreadsheetCell {
    Int(i32),
    Float(f32),
    Text(String),
}

fn main() {
    let vec: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(1.0);
    v3.push(2.0);
    v3.push(3.0);
    v3.push(4.0);

    println!("vec {vec:?}, v2 {v2:?}, v3 {v3:?}");

    let third = &v3[2];
    println!("third = {third}");

    let fourthOpt: Option<&f32> = v3.get(3);
    match fourthOpt {
        Some(fourthOpt) => println!("this is the fourth elemt {fourthOpt}"),
        Nome => println!("there is no fourth element"),
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("leo")),
    ];

    let s = "leo";
    let data = s.to_string();

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{:#?}", s1);

    //Don't use index on string, instead use charr function
    for c in "Зд".chars() {
        println!("{c}");
    }
}
