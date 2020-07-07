use std::thread;
use std::time::Duration;

fn main() {

    let handle = thread::spawn(|| {
        for ii in 1..10 {
            println!("새 스레드 : {}", ii);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for ii in 1..10 {
        println!("주 스레드 : {}", ii);
        thread::sleep(Duration::from_millis(1));
    }
}
