use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use clap::Parser;
use tiny_http::{Method, Request, Response};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args{
    /// The IPv4 address for the main PC. Does not include port.
    #[arg(short, long)]
    ip: String,
    /// The MAC(physical) address for the main PC
    #[arg(short, long)]
    mac: String
}

fn main() {
    let args = Args::parse();

    let server = tiny_http::Server::http(SocketAddr::from(([0, 0, 0, 0], remote_tools::SERVER_PORT))).unwrap();

    let remote_ip = SocketAddr::new(IpAddr::from_str(&args.ip).unwrap(), remote_tools::SERVER_PORT);
    let remote_mac = remote_tools::parse_mac_from_str(args.mac).unwrap();

    println!("[OK] Server listening!");
    for request in server.incoming_requests() {
        println!("[OK] Incoming request: {request:?}");
        match handle_request(request, &remote_ip, &remote_mac) {
            Ok(_) => {},
            Err(e) => println!("[ERROR] Error handling request: {e:?}")
        }
    }
}

fn handle_request(request: Request, remote_ip: &SocketAddr, remote_mac: &[u8;6]) -> std::io::Result<()> {
    let acao = tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap();

    match request.method() {
        Method::Get => match request.url() {
            "/" => {
                println!("[OK] Served HTTP.");
                request.respond(
                    Response::from_data(remote_tools::HTML)
                        .with_header(acao)
                        .with_status_code(200)
                )?
            },
            _ => request.respond(
                Response::empty(404)
                    .with_header(acao)
            )?
        },
        Method::Post => match request.url() {
            "/shutdown" => {
                match shutdown_remote(remote_ip) {
                    Ok(_) => {
                        println!("[OK] Shutting down remote!");
                        request.respond(
                            Response::empty(200)
                                .with_header(acao)
                        )?
                    },
                    Err(err) => {
                        println!("[ERROR] Error shutting down remote: {err:?}");
                        request.respond(
                            Response::empty(500)
                                .with_header(acao)
                        )?
                    }
                }
            },
            "/startup" => {
                let packet = wake_on_lan::MagicPacket::new(remote_mac);

                match packet.send() {
                    Ok(_) => {
                        println!("[OK] Successfully sent magic package to {remote_mac:?}");
                        request.respond(
                            Response::empty(200)
                                .with_header(acao)
                        )
                    },
                    Err(err) => {
                        println!("[ERROR] Error sending magic package to {remote_mac:?}: {err:?}");
                        request.respond(
                            Response::from_string(err.to_string())
                                .with_status_code(500)
                                .with_header(acao)
                        )
                    }
                }?
            }
            _ => request.respond(Response::empty(501))?
        },
        _ => request.respond(Response::empty(501))?
    }

    Ok(())
}

fn shutdown_remote(remote_ip: &SocketAddr) -> std::io::Result<()> {
    let mut stream = std::net::TcpStream::connect(remote_ip)?;

    stream.write_all(&[remote_tools::OP_SHUTDOWN])?;

    let mut status = [0];
    stream.read_exact(&mut status)?;

    match status[0] {
        remote_tools::OP_OK => {
            Ok(())
        },
        _ => {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Pc did not respond with ok."))
        }
    }
}
