use std::rc::Rc;
mod utils; use utils::report_rc_count;

enum List {
    Cons(i32, Rc<List>),
    Nil
}
use List::{Cons, Nil};


fn main() {
    let a = Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Nil)))));  // シングルスレッドでは、複数変数でのデータ共有はこうやる
    report_rc_count("after creating a", &a);

    let _b = Cons(1, Rc::clone(&a));  // deep copy しないので .clone() よりパフォーマンスがよい
    report_rc_count("after creating b", &a);
    {
        let _c = Cons(2, Rc::clone(&a));
        report_rc_count("after creating c in scope", &a);
    }
    report_rc_count("after c drops the scope", &a);

}