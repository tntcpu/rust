use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
fn main() {
    let m = MyBox::new(String::from("Rust"));
    // let m = String::from("Rust");
    // hello(&(*m)[..]);
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!")
}
