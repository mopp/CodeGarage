enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}


trait Sugoi {
    fn say(&self) -> i32 {
        println!("sugoi !!!");
        0
    }
}


impl Sugoi for Message {
}


fn main() {
    let e = Message::Write("Hello".to_string());
    e.say();
}
