use  std::{
    sync::mpsc,  // MSPC: "Malutiple Producer, Single Reciever"
    thread, time::Duration,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    let another_tx = tx.clone();
    thread::spawn(move || {
        let msgs = [
            "Hi,", "from", "the", "thread"
        ];
        for msg in msgs {
            another_tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }        
    });

    thread::spawn(move || {  // tx から send するために move
        let msgs = [
            "more,", "message", "for", "you"
        ];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let recieved = rx.recv();  // これだと最初の Ok("Hi") を受け取って終了
    // println!("recieved {:?}", recieved);
    for recv in rx {  // as an iterator of recieved values UNWRAPED
        println!("recieved: {}", recv);
    }
}