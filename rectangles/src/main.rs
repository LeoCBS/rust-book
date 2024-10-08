fn main() {

    let width = 30;
    let heigth = 50;
    println!("the rectangle area is {}", area(width, heigth));
    
    let rect = (30,50);
    println!("the rectangle area is {}", area_tuple(rect));
   
    let scale = 2;
    let rect_struct = Rectangle{
        width:dbg!(30 * scale),
        heigth:50,
    };
    //println!("the rectangle area is {}", area_struct(rect_struct));
    println!("the rectangle struct is {rect_struct:#?}");
}

fn area(width: u32, heigth:u32) -> u32{
    width*heigth
}

fn area_tuple(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    heigth: u32,
}

fn area_struct(rect:Rectangle)-> u32{
    rect.width * rect.heigth
}
