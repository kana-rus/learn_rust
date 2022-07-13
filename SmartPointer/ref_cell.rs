pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
} impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let per_of_max = (self.value as f64) / (self.max as f64);
        
        if 0.75 <= per_of_max {
            self.messenger.send(
                if per_of_max < 0.9 {
                    "Warning: You've used up over 75% of your quota!"
                } else if per_of_max < 1.0 {
                    "Urgent warning: You've used up over 90% of your quota!"
                } else {
                    "Error: You are over your quota!"
                }
            )
        } 
    }
}

use std::{cell::RefCell, rc::Rc};


#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    } impl MockMessenger {
        fn new() -> Self {
            MockMessenger{
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {// ここを可変参照にしたくないが Mockでは可変性のあるメソッドを扱いたい
            self.sent_messages
                .borrow_mut()  // これによって、内部の値を更新しつつ、対外的には不変のように振る舞う
                .push(String::from(msg));
        }
    }

    #[test]
    fn expect_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(100);
        assert_eq!(
            mock_messenger.sent_messages
                          .borrow()  // 対外的に不変値として扱われる。通常コンパイラがやる諸々のチェックが実行時に回される [[内部可変性パターン]]
                                     // 代償として、当然パフォーマンスは下がる
                          .len(),
            1
        );
    }
}


#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
fn main() {
    let val = Rc::new(RefCell::new(5));

    let a = Rc::new(
        Cons(Rc::clone(&val), Rc::new(Nil))
    );

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    println!("
a created: {:?}
b created: {:?}
c created: {:?}
    ", a, b, c);
/*
a created: Cons(RefCell { value: 5 }, Nil)
b created: Cons(RefCell { value: 6 }, Cons(RefCell { value: 5 }, Nil))
c created: Cons(RefCell { value: 10 }, Cons(RefCell { value: 5 }, Nil))
*/

    *
    val
    // Rc<RefCell<T>> -> RefCell<T> (自動参照外し / 型強制)
    .borrow_mut() // RefCell<T> -> RefMut<T>
    // * で RefMut<T> -> mut T
    += 10;
    
    println!("
a after: {:?}
b after: {:?}
c after: {:?}
    ", a, b, c);
}
/*
a after: Cons(RefCell { value: 15 }, Nil)
b after: Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after: Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
*/
