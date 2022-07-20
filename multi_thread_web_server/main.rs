/**
 * 現状、改行記号の byte 処理の関係で
 * たぶん Linux でしかちゃんと動かない
 */

use std::{
    net::{TcpListener, TcpStream},
    io::{Read, Write},
    fs::File, thread, time::Duration,
};


fn handle_connection(mut stream: TcpStream) {
    const BUFF_SIZE: usize = 1024;
    let mut buff = [0_u8; BUFF_SIZE];
    stream.read(&mut buff)
          .expect("failed to read stream");

    /* HTTP response format :
    [http-version] [status-code] [reason-phrase] \r\n
    [headers] \r\n
    [message-body]

    *example (NO [headers], NO [message-body]) :
    HTTP/1.1 200 OK\r\n\r\n
    */

    let request_status = {
        let mut end_of_status = BUFF_SIZE;  // init
        for i in 0..BUFF_SIZE {
            if buff[i]   == b"\r"[0]    // 13 on Linux
            && buff[i+1] == b"\n"[0] {  // 10 on Linux
                if i == 0 {
                    println!("starts with \"r\"");
                    panic!();
                }
                end_of_status = i - 1;
                break;
            }
        }

        if end_of_status == BUFF_SIZE {
            println!("not contain valid status");
            panic!();
        }

        &buff[..=end_of_status]
    };

    let (status_line, file_path)
        = match request_status {
              b"GET / HTTP/1.1"      => ("HTTP/1.1 200 OK\r\n\r\n", "multi_thread_web_server/hello.html"),
              b"GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK\r\n\r\n", "multi_thread_web_server/sleep.html")
              },
              _                      => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "multi_thread_web_server/404.html"),
          };
    
    let mut content = String::new();
    File::open(&file_path)
        .expect(&format!("failed to read file: {}", file_path))
        .read_to_string(&mut content)
        .expect("failed to write html to string");
    stream.write(
        format!("{}{}", status_line, content).as_bytes()
    ).expect("failed to write response to stream");
    stream.flush().expect("failed to flushing stream");
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind address");

    for stream in listener.incoming() {
        let stream = stream.expect("failed to unwrap stream");
        handle_connection(stream);
    }
}
