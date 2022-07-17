use std::{
    rc::Rc,
    cell::RefCell,
};

#[allow(unused)]
pub fn rc<T: Sized>(item: T) -> Rc<T> {
    Rc::new(item)
}

#[allow(unused)]
pub fn ref_cell<T: Sized>(item: T) -> RefCell<T> {
    RefCell::new(item)
}

#[allow(unused)]
pub fn rc_clone<T: Sized>(target: &Rc<T>) -> Rc<T> {
    Rc::clone(target)
}

#[allow(unused)]
pub fn report_rc_count<T>(situation: &'static str, rc: &Rc<T>) {
    const SITUATION_DESCRIBING_LEN: usize = 25;
    let l = &situation.len();
    if l > &SITUATION_DESCRIBING_LEN {
        println!(
            "please describe situation within {} charactors including whitespaces!",
            SITUATION_DESCRIBING_LEN
        );
    }

    print!("{}", situation);
    for _ in 0..(SITUATION_DESCRIBING_LEN - l) {
        print!(" ");
    }
    println!("  |  {}", Rc::strong_count(rc));
}