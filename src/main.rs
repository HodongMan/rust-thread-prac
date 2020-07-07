use std::thread;

fn main() {

    let value = vec![1, 2, 3];

    let handle = thread::spawn( move || {
        println!("벡터 : {:?}", value);
    });

    handle.join().unwrap();
}
