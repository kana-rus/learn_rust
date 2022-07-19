use std::{
    time::Duration,
    sync::{
        Mutex,  /* Mutual Exclusion (相互排他)
        - RefCell のように内部可変性を提供
        - RefCell 同様、コンパイラはロジックエラーから守ってくれない
        - Rc<RefCell<T>> でいう循環参照のように、Mutex はデッドロックを生む可能性がある */
        
        Arc  /* Rc のマルチスレッド版
        - スレッド安全
        - パフォーマンスは犠牲になる */
    },
    thread,
    // rc::Rc,
    /*
     * 1. lock a data
         * 2. execute some process with the locked_object
     * 3. unlock the locked_object to Mutex(data)
    */
};

fn main() {
    /*
    let m5 = Mutex::new(5);
    {
        let mut num = m5.lock()
                                        .expect("error unwraping Result");
        *num = 6;
    }  // AUTOMATICALY unlocked MutexGuard(m5) to Mutex(m5)
       // when MutexGuard DROPPED
    println!("m = {:?}", m5);
    */

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {            
            let mut num = counter.lock().unwrap();
            let mut report = format!("incrememented {} -> ", &num);
            *num += 1;
            report += &num.to_string();
            println!("{}", report);
            thread::sleep(Duration::from_secs(1));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("final count: {}", counter.lock().unwrap());
}