use std::{
    cell::RefCell,
    rc::Rc,
};
mod utils; use utils::{
    rc, ref_cell, report_rc_count,
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
} impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
use List::{Cons, Nil};


fn main() {
    let a = rc(Cons(5, ref_cell(rc(Nil))));
    report_rc_count("a's initial count", &a);
    println!("a's next item (tail) is: {:?}", a.tail());

    let b = rc(Cons(10, ref_cell(Rc::clone(&a))));
    // b は a を指している
    report_rc_count("a's count after b creation", &a);
    report_rc_count("b's initial count", &b);
    println!("b's next item (tail) is: {:?}", b.tail());

    if let Some(a_link) = a.tail() {
        // a_link ... a の Cons の後ろ: RedCell(Rc(Nil))

        // a_link.borrow_mut() ... RefMut(Rc(Nil))
        // *a_link.borrow_mut() ... (内部) mut Rc(Nil)
        *a_link.borrow_mut() = Rc::clone(&b);
        // これで a が b を指し、循環参照！！
    }
    report_rc_count("b's count after changing a", &b);
    report_rc_count("a's count after changing a", &a);
}