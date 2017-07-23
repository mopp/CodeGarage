mod parser;

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
                use parser::ParseResult::*;
                let mut buf = Vec::new();
                loop {
                    let mut b = [0; 1024];
                    let n = stream.read(&mut b)?;

                    if n == 0 {
                        return Ok(());
                    }

                    buf.extend_from_slice(&b[0..n]);
                    match parser::parse(buf.as_slice()) {
                        Partial => continue,
                        Error => {
                            return Ok(());
                        },
                        Complete(req) => {
                            write!(stream, "OK {}\r\n", req.0)?;
                            return Ok(());
                        },
                    };
                }
            }
        );
    }

    Ok(())
}

fn main() {
    match server_start() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
