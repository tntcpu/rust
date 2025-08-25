fn main() {
    let mut num = 5;

    let _r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;
}
