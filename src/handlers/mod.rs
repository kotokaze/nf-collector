mod nfv5_handler;
pub use nfv5_handler::Nfv5Handler;

use crate::nfmsg::NfMessage;

use anyhow::Result;
use std::{fmt::Display, net::SocketAddr};

pub trait Handler: Send + Display {
    fn clone_box(&self) -> Box<dyn Handler>;
    fn handle(&self, buf: &Vec<u8>, size: usize, addr: SocketAddr) -> Result<Vec<NfMessage>>;
}

impl Clone for Box<dyn Handler> {
    fn clone(&self) -> Box<dyn Handler> {
        self.clone_box()
    }
}
