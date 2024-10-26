use std::collections::HashMap;

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
    //println!("{:#?}", s1);

    //Don't use index on string, instead use charr function
    for c in "Зд".chars() {
        println!("{c}");
    }

    hash_map();
}

fn hash_map(){
    let mut scores = HashMap::new();

    let field_name = String::from("blue");
    let field_value = 10;

    //hash maps take variable ownership, moved
    scores.insert(field_name, field_value);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    //iteration
    for (key, value) in &scores{
        println!("{key}: {value}");
    }

    scores.entry(String::from("blue")).or_insert(50);

    //iteration
    for (key, value) in &scores{
        println!("{key}: {value}");
    }

    let text = "hello world worderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    println!("{map:?}");

}
