use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil
}

use List::{Cons, Nil};


fn main() {
    let report_rc_count = |situation: &'static str, rc: &Rc<List>| {
        const SSTUATION_DESCRIBING_LEN: usize = 25;
        let l = &situation.len();
        if l > &SSTUATION_DESCRIBING_LEN {
            println!(
                "please describe situation within {} charactors including whitespaces!",
                SSTUATION_DESCRIBING_LEN
            );
        }

        print!("{}", situation);
        for _ in 0..(SSTUATION_DESCRIBING_LEN - l) {
            print!(" ");
        }
        println!("  |  {}", Rc::strong_count(rc));
    };

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