use std::fmt;

enum State {
    Running,
    Waiting,
    Sleeping,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("In fmt()");
        write!(f, "{}", self.to_string())
    }
}


fn main() {
    println!("{}", State::Running);
}
