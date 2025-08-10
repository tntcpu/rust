enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn main() {
    // let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    // let msg = Message::Move { x: 160, y: 255 };
    // let msg = Message::Quit;

    match msg {
        // Message::Quit => {
        //     println!("The Quit variant has no data to destructure.");
        // }
        // Message::Move { x, y } => {
        //     println!("Move in the x direction {} and in the y direction {}", x, y);
        // }
        // Message::Write(text) => {
        //     println!("Text message: {}", text);
        // }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value{v}")
        }
        _ => {}
    }
}
