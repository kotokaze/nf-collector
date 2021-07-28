mod nfv5;
pub use nfv5::{Nfv5Header, Nfv5HeaderBuilder, Nfv5Record, Nfv5RecordBuilder};

use derive_builder::Builder;
use std::net::SocketAddr;

#[derive(Builder, Debug)]
pub struct NfMessage {
    // Time stamp
    #[builder(setter(into, strip_option), default)]
    datetime: Option<String>,

    #[builder(setter(into, strip_option), default)]
    exporter_addr: Option<SocketAddr>,

    /* Netflow V5 */
    // Header
    #[builder(setter(into, strip_option), default)]
    nfv5_header: Option<Nfv5Header>,

    // Record
    #[builder(setter(into, strip_option), default)]
    nfv5_record: Option<Nfv5Record>,
}
