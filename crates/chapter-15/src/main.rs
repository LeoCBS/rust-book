enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    /*    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);*/
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    /*    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }*/
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
