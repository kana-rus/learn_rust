use std::{
    thread,
    time::Duration,
};


fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("{} from thread", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..10 {
        println!("{} from main", i);
        thread::sleep(Duration::from_millis(10));
    }
}