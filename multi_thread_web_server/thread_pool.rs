use std::{
    thread,
    sync::{mpsc, Arc, Mutex},
};


type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,  // これを用意しておかないとスレッドを終了できない (job を探して無限ループしてしまう)
}

/**
 * 通常の thread::JoinHandle<()> は、生成され次第渡されたクロージャーで
 処理を行うことを前提にしている。
 * 今回はそうではなく、出番が来るまで待機し、呼ばれたら処理を開始してほしいので、
 thread::JoinHandle<()> をラップする struct Worker を用意する。
 */
#[allow(unused)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>  /* ThreadPool の Drop で thread の所有権を奪う
    必要があるので、take() でそれを実現するために Option で包む */
} impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = reciever.lock()
                                  .expect("failed to lock reciever")
                                  .recv()
                                  .expect("failed to recieve message");

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing", &id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", &id);
                        break;
                    },
                }
                
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/**
* ThreadPool が channel を生成する
* ((Sender)) ThreadPool 1------>N ((Reciever)) Worker
* ThreadPool で処理を表すクロージャーを受け取って Job に持たせ、
channel を通して Job を Worker に渡す. 各 Worker が Job を処理する

* channel 自体は１つで、全 Worker が１つの reciever から Job を受け取る.
そのため、複数 thread 間で reciever の所有権を共有する必要がある.
    -> reciever を Arc で包む
* キューに積まれた Job を各 Worker が取り出すことになるので、マルチスレッド
の文脈で安全な可変性が必要.
    -> reciever を Mutex で包む
       (一度に起こる変化は１つの Worker が１つの Job を取ることだけであると保証する)
*/
#[allow(unused)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
} impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciever) = mpsc::channel::<Message>();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
        /*
            thread::spawn() のトレイト境界に合わせて FnOnce,
            main thread から別の thread にクロージャーを送る必要があるので Send,
            thread の実行 (リクエストの処理) にかかる時間はコンパイル時点で分からないので 'static
        */
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).expect("failed to send job");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        for _ in &self.workers {
            self.sender.send(Message::Terminate)
                       .expect("failed to send Terminate");
        }

        // Terminate を送りつけておいたので、以下の join() によって各 Worker は break する

        println!("Shutting down all workers");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            /*
            ---- wroker.thread を Option で包まない場合

            worker.thread.join()
                         .expect("failed to join thread");

            となるが、join() は move を伴う処理なので、
            worker が可変参照である以上エラーになる
            */
            if let Some(thread) = worker.thread.take() {
            // take() で thread を woiker --> Some の中の thread変数 に move
                thread.join()
                      // thread を move してきたので join() できる
                      .expect("failed to join thread");
            }
        }
    }
}