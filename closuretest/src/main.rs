use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // let mut borrows_mutably = || list.push(7);
    // // println!("From closure {:?}", list);
    // borrows_mutably();
    // println!("After calling closure: {:?}", list);
}
