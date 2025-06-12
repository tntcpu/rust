use std::sync::{Arc,Mutex};
use std::thread;
fn main() {
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }
    thread::sleep(std::time::Duration::from_millis(100));
    println!("Result: {}", *counter.lock().unwrap());
}
