use std::sync::Mutex;

fn mutexFunc() {

    let mutex = Mutex::new(5);

    {
        let mut num = mutex.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", mutex);
}