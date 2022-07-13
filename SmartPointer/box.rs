use std::ops::Deref;

use List::{Cons, Nil};


fn main() {
    // 再帰的な定義
    let _list = Cons(1, boxed(Cons(2, boxed(Cons(4, boxed(Nil))))));

    let x = 5;
    let y = boxed(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5); // Deref trait による *
    assert_eq!(y, boxed(5));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    /*
    &MyBox<String> -> &String  // by MyBox's Deref impl
                   -> &str  // by String's Deref impl, "参照外し型強制"
    */
}


enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn boxed<T: Sized>(item: T) -> Box<T> {
    Box::new(item)
}

struct MyBox<T> (T); // tupple of only 1 element
impl<T> MyBox<T> {
    fn new(item: T) -> Self {
        MyBox(item)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}