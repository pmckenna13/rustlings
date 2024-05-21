// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// DONE

#[derive(Debug)]
enum Message {
    // This enum has four variants with different types:

    // Quit has no data associated with it at all.
    // Move has named fields, like a struct does.
    // Echo includes a single String.
    // ChangeColor includes three i32 values.
    Move {x:u8, y:u8},
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
