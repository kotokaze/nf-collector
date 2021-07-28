use derive_builder::Builder;
use std::net::IpAddr;

#[derive(Builder, Debug, Clone)]
pub struct Nfv5Header {
    // NetFlow export format version number
    #[builder(setter(into, strip_option), default)]
    version: Option<u16>,

    // Number of flows that are exported in this packet (1-30)
    #[builder(setter(into, strip_option), default)]
    count: Option<u16>,

    // Current time since the export device started [msec]
    #[builder(setter(into, strip_option), default)]
    sysup_time: Option<u32>,

    // Current count of seconds since 0000 Coordinated Universal Time 1970
    #[builder(setter(into, strip_option), default)]
    unix_secs: Option<u32>,

    // Residual nanoseconds since 0000 Coordinated Universal Time 1970
    #[builder(setter(into, strip_option), default)]
    unix_nsecs: Option<u32>,

    // Seq counter of total flows seen on the export device
    #[builder(setter(into, strip_option), default)]
    flow_sequence: Option<u32>,

    // Type of flow-switching engine
    #[builder(setter(into, strip_option), default)]
    engine_type: Option<u8>,

    // Slot number of flow-switching engine
    #[builder(setter(into, strip_option), default)]
    engine_id: Option<u8>,

    // First two bits hold the sampling mode
    // remaining 14 bits hold value of sampling interval
    #[builder(setter(into, strip_option), default)]
    sampling: Option<u16>,
}

#[derive(Builder, Debug, Clone)]
pub struct Nfv5Record {
    // Source IP address
    #[builder(setter(into, strip_option), default)]
    src_addr: Option<IpAddr>,

    // Destination IP address
    #[builder(setter(into, strip_option), default)]
    dst_addr: Option<IpAddr>,

    // IP address of next hop router
    #[builder(setter(into, strip_option), default)]
    nexthop_addr: Option<IpAddr>,

    // SNMP index of input interface
    #[builder(setter(into, strip_option), default)]
    in_if: Option<u16>,

    // SNMP index of output interface
    #[builder(setter(into, strip_option), default)]
    out_if: Option<u16>,

    // Packets in flow
    #[builder(setter(into, strip_option), default)]
    d_pkts: Option<u32>,

    // Total number of Layer 3 bytes in the packets of the flow
    #[builder(setter(into, strip_option), default)]
    d_octets: Option<u32>,

    // SysUptime at start of flow
    #[builder(setter(into, strip_option), default)]
    first_time: Option<u32>,

    // SysUptime at end of flow
    #[builder(setter(into, strip_option), default)]
    last_time: Option<u32>,

    // TCP/UDP source port number or equivalent
    #[builder(setter(into, strip_option), default)]
    src_port: Option<u16>,

    // TCP/UDP destination port number or equivalent
    #[builder(setter(into, strip_option), default)]
    dst_port: Option<u16>,

    // Cumulative OR of TCP flags
    #[builder(setter(into, strip_option), default)]
    tcp_flags: Option<u8>,

    // IP protocol type (for example, TCP = 6; UDP = 17)
    #[builder(setter(into, strip_option), default)]
    prot: Option<u8>,

    // IP type of service (ToS)
    #[builder(setter(into, strip_option), default)]
    tos: Option<u8>,

    // Autonomous system number of the source, either origin or peer
    #[builder(setter(into, strip_option), default)]
    src_as: Option<u32>,

    // Autonomous system number of the destination, either origin or peer
    #[builder(setter(into, strip_option), default)]
    dst_as: Option<u32>,

    // Source address prefix mask bits
    #[builder(setter(into, strip_option), default)]
    src_mask: Option<u8>,

    // Destination address prefix mask bits
    #[builder(setter(into, strip_option), default)]
    dst_mask: Option<u8>,
}
