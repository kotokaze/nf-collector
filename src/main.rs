// use std::error::Error;
use tokio::net::UdpSocket;
use nf_collector::handlers::{Handler, Nfv5Handler};
use nf_collector::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:9996";
    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on {}", socket.local_addr()?);

    let mut handlers: Vec<Box<dyn Handler>> = Vec::new();
    let nf_v5_handler = Box::new(Nfv5Handler::new());
    handlers.push(nf_v5_handler);

    println!("handlers: [{}]", handlers
        .iter()
        .map(|h| h.to_string())
        .collect::<Vec<String>>()
        .join(", ")
    );

    let server = Server{
        socket: socket,
        buf: vec![0u8; 4096],
        handlers: handlers,
    };

    server.exec().await?;
    Ok(())
}
