use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len(); //1
    let ptr = values.as_mut_ptr(); //2

    assert!(mid <= len); //3

    unsafe {
        //4
        (
            slice::from_raw_parts_mut(ptr, mid),                //5
            slice::from_raw_parts_mut(ptr.add(mid), len - mid), //6
        )
    }
}

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}
