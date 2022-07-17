use std::{
    rc::Rc,
    cell::RefCell,
};
mod utils; use utils::{
    ref_cell, rc//, rc_clone,
};

/*
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = rc(Node {
        value: 3,
        children: ref_cell(vec![]),
    });
    let branch = rc(Node {
        value: 5,
        children: ref_cell(vec![rc_clone(&leaf)])
    });
}
*/
/*
 * この時点で leaf の Node には
 * leaf, brach という２つの所有者がいる
 * 
 * branch.children によって branch (親) から leaf (子) にアクセスできるが、
 * 逆は無理
 * 
 * leaf からも brach が見えるようにしたいが、./simple_circulr_ref.rs のような
 * 循環参照はまずい
 */

// そこで weak_count を使う

use std::rc::Weak;
// Rc > Weak

#[derive(Debug)]
struct Node {
    _value: i32,
    parent: RefCell<Weak<Node>>,  // 子から親を参照できるが 所有はしない
    _children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = rc(Node {
        _value: 3,
        parent: ref_cell(Weak::new()),
        _children: ref_cell(vec![]),
    });
    println!(
        // leafのstrong_count = {}, weak_count = {}
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            _value: 5,
            parent: RefCell::new(Weak::new()),
            _children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        println!(
            // branchのstrong_count = {}, weak_count = {}
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            // branchのstrong_count = {}, weak_count = {}
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}