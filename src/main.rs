mod constants;

use std::str;
use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;


struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    async fn run(self) -> Result<(), io::Error> {
      
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {

            to_send = Some(socket.recv_from(&mut buf).await?);
            if let Some((size, peer)) = to_send {
                println!("Receive {:?} from {}", str::from_utf8(&buf[..size]).unwrap(), peer);
            }

        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3478".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}