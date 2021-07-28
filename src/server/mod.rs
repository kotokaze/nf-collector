use crate::handlers::Handler;

use anyhow::Result;
use tokio::net::UdpSocket;

pub struct Server {
    pub socket: UdpSocket,
    pub buf: Vec<u8>,
    pub handlers: Vec<Box<dyn Handler>>,
}

impl Server {
    pub async fn exec(self) -> Result<()> {
        let Server {
            socket,
            mut buf,
            handlers,
        } = self;

        loop {
            match socket.recv_from(&mut buf).await {
                Ok((size, addr)) => {
                    let cbuf = buf.clone();
                    let chandlers = handlers.clone();

                    std::thread::spawn(move || {
                        for handler in chandlers.iter() {
                            match handler.handle(&cbuf, size, addr) {
                                Ok(flows) => {
                                    for flow in flows.iter() {
                                        dbg!(&flow);
                                    }
                                },
                                Err(e) => {
                                    eprintln!("{}", e);
                                },
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("{}", e);
                },
            };
        }
    }
}
