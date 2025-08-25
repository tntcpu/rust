//! # Adder
//!
//! `adder`是一个加法工具
//! 这个工具被用来简化特定的计算操作
///将传入的数字加2
///
///# Examples
///
///```
///let arg = 5;
///let answer = adder::add_two(arg);
///
///assert_eq!(6, answer);
///```
pub fn add_two(a: i32) -> i32 {
    internal_add(a, 2)
}
fn internal_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {

        // assert!(smaller.can_hold(&larger));
    }
}
