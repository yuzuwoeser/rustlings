// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Echo,
    Move,
    Quit,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
