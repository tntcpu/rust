#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut closure_call_num = 0;
    list.sort_by_key(|r| {
        closure_call_num += 1;
        r.width
    });
    println!("{:#?}, sorted in {closure_call_num} operations", list);
}
