use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("안녕하씸미까"),
            String::from("무딜량호흡"),
            String::from("데프트"),
            String::from("임미다"),
            String::from("ㅡㅅㅡ"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("수신 : {}", received);
    }
}