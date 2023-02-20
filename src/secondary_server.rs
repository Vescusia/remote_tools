use std::io::{Read, Write};
use std::net::SocketAddr;
use std::process::Command;

use remote_tools::*;

fn main() {
    let server = std::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], SERVER_PORT))).unwrap();

    println!("[OK] Server listening!");
    for stream in server.incoming() {
        let stream = stream.unwrap();
        println!("[OK] Stream incoming! {stream:?}");
        match handle_stream(stream) {
            Ok(_) => {},
            Err(e) => println!("[ERROR] Error handling stream: {e:?}")
        }
    }
}

fn handle_stream(mut stream: std::net::TcpStream) -> std::io::Result<()> {
    let mut op = [0];
    stream.read_exact(&mut op)?;
    let op = op[0];

    match op {
        OP_SHUTDOWN => {
            println!("[OK] Shutting down!");

            match if cfg!(windows) {
                Command::new("cmd")
                    .arg("/C")
                    .args(["shutdown", "/s", "/t 0"])
                    .spawn()
            } else {
                Command::new("ls")
                    .spawn()
            } {
                Ok(_) => stream.write_all(&[OP_OK])?,
                Err(err) => {
                    println!("[ERROR] Error shutting down! {err}");
                    stream.write_all(&[OP_NOT_OK])?
                }
            }
        },
        _ => {
            stream.shutdown(std::net::Shutdown::Both)?;
        }
    }

    Ok(())
}
