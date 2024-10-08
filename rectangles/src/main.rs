fn main() {

    let width = 30;
    let height = 50;
    println!("the rectangle area is {}", area(width, height));
    
    let rect = (30,50);
    println!("the rectangle area is {}", area_tuple(rect));
   
    let scale = 2;
    let rect_struct = Rectangle{
        width:dbg!(30 * scale),
        height:50,
    };
    //println!("the rectangle area is {}", area_struct(rect_struct));
    println!("the rectangle struct is {rect_struct:#?}");
    println!("the rectangle area is {}", rect_struct.area());
    println!("the rectangle width nonzero {}", rect_struct.width);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    

    println!("Using associated function {:#?}", Rectangle::square(10));

}




fn area(width: u32, height:u32) -> u32{
    width*height
}

fn area_tuple(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn area_struct(rect:Rectangle)-> u32{
    rect.width * rect.height
}

impl Rectangle{
    fn area(&self)-> u32{
        self.width * self.height
    }

    fn width(&self) -> bool{
        self.width > 0
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool{
        self.width > other_rect.width && self.height > other_rect.height
    }

    fn square(size: u32) -> Self{
        Self{
            width:size,
            height: size,
        }
    }
}
