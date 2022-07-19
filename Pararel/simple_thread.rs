use std::{
    thread,
    time::Duration,
};


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from thread", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    handle.join().unwrap();  // wait for handle finishing

    for i in 1..10 {
        println!("{} from main", i);
        thread::sleep(Duration::from_millis(10));
    }


    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // thread はいつまで存在するかコンパイラには分からないので、 move しなければならない
        println!("v is {:?}", v);
    });
    handle.join().unwrap();
}