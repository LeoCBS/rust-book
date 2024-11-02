fn main() {
    let int_vec = vec![100, 150, 50, 85, 40];
    let largest = largest(&int_vec);
    println!("largest number in the list is {}", largest);
    let p = Point { x: 1, y: 2.0 };
    println!("point x generic {}", p.x())
}

fn largest<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
