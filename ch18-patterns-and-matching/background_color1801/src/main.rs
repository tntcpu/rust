fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    //1
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background"); //2
    //3
    } else if is_tuesday {
        println!("Tuesday is green day!"); //4
    //5
    } else if let Ok(age) = age {
        //6
        if age > 30 {
            println!("Using purple as the background color"); //7
        } else {
            println!("Using orange as the background color"); //8
        }
    } else {
        //9
        println!("Using blue as the background color"); //10
    }
}
