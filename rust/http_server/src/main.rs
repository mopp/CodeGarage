use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};
use std::io;


fn server_start() -> io::Result<()> {
    let lis = TcpListener::bind("127.0.0.1:8080")?;

    for stream in lis.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("An error occured while accepting a connection: {}", e);
                continue;
            }
        };

        let _ = thread::spawn(
            move || -> io::Result<()> {
                loop {
                    let mut buffer = [0; 1024];
                    let count = stream.read(&mut buffer)?;
                    if count == 0 {
                        return Ok(());
                    } else {
                        stream.write(&buffer[0..count])?;
                    }
                }});
    }

    Ok(())
}

fn main() {
    match server_start() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
