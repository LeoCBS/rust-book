fn main() {

    let width = 30;
    let heigth = 50;
    println!("the rectangle area is {}", area(width, heigth));
}

fn area(width: u32, heigth:u32) -> u32{
    width*heigth
}
