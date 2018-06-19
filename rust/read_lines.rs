use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    // let part_a = lines.take_while(|line| !line.as_ref().unwrap().is_empty());

    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                println!("skip !");
            } else {
                println!("todo something with {}", line);
            }
        } else {
            println!("break");
            break;
        }
    }
}
