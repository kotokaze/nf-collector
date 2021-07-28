use super::Handler;
use crate::nfmsg::{NfMessage, Nfv5Header, Nfv5Record};
use crate::nfmsg::{NfMessageBuilder, Nfv5HeaderBuilder, Nfv5RecordBuilder};
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use chrono::Utc;
use std::fmt::{Display, Formatter};
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone, Debug)]
pub struct Nfv5Handler {}

impl Nfv5Handler {
    pub fn new() -> Nfv5Handler {
        Nfv5Handler {}
    }
}

impl Display for Nfv5Handler {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Nfv5Handler")
    }
}

impl Handler for Nfv5Handler {
    fn clone_box(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }

    fn handle(
        &self,
        buf: &Vec<u8>,
        _size: usize,
        exporter_addr: SocketAddr,
    ) -> Result<Vec<NfMessage>> {
        let mut reader = Cursor::new(buf.as_slice());

        let datetime = Utc::now();

        /* Netflow v5 Header */
        let version = reader.read_u16::<BigEndian>()?;
        let count = reader.read_u16::<BigEndian>()?;
        let sysup_time = reader.read_u32::<BigEndian>()?;
        let unix_secs = reader.read_u32::<BigEndian>()?;
        let unix_nsecs = reader.read_u32::<BigEndian>()?;
        let flow_sequence = reader.read_u32::<BigEndian>()?;
        let engine_type = reader.read_u8()?;
        let engine_id = reader.read_u8()?;
        let sampling_interval = reader.read_u16::<BigEndian>()?;

        let mut nfmsgs = Vec::with_capacity(count as usize);
        let mut v5header: Nfv5Header;
        let mut v5record: Nfv5Record;

        for _ in 0..count {
            let mut nfmsg_builder = NfMessageBuilder::default();
            let mut v5header_builder = Nfv5HeaderBuilder::default();
            let mut v5record_builder = Nfv5RecordBuilder::default();

            /* Netflow v5 Record */
            let srcaddr = IpAddr::V4(Ipv4Addr::from(reader.read_u32::<BigEndian>()?));
            let dstaddr = IpAddr::V4(Ipv4Addr::from(reader.read_u32::<BigEndian>()?));
            let nexthop = IpAddr::V4(Ipv4Addr::from(reader.read_u32::<BigEndian>()?));
            let input = reader.read_u16::<BigEndian>()?;
            let output = reader.read_u16::<BigEndian>()?;
            let d_pkts = reader.read_u32::<BigEndian>()?;
            let d_octets = reader.read_u32::<BigEndian>()?;
            let first = reader.read_u32::<BigEndian>()?;
            let last = reader.read_u32::<BigEndian>()?;
            let srcport = reader.read_u16::<BigEndian>()?;
            let dstport = reader.read_u16::<BigEndian>()?;
            let tcp_flags = reader.read_u8()?;
            let prot = reader.read_u8()?;
            let tos = reader.read_u8()?;
            let src_as = reader.read_u32::<BigEndian>()?;
            let dst_as = reader.read_u32::<BigEndian>()?;
            let src_mask = reader.read_u8()?;
            let dst_mask = reader.read_u8()?;

            v5header_builder
                .version(version)
                .count(count)
                .sysup_time(sysup_time)
                .unix_secs(unix_secs)
                .unix_nsecs(unix_nsecs)
                .flow_sequence(flow_sequence)
                .engine_type(engine_type)
                .engine_id(engine_id)
                .sampling(sampling_interval);

            v5record_builder
                .src_addr(srcaddr)
                .dst_addr(dstaddr)
                .nexthop_addr(nexthop)
                .in_if(input)
                .out_if(output)
                .d_pkts(d_pkts)
                .d_octets(d_octets)
                .first_time(first)
                .last_time(last)
                .src_port(srcport)
                .dst_port(dstport)
                .tcp_flags(tcp_flags)
                .prot(prot)
                .tos(tos)
                .src_as(src_as as u32)
                .dst_as(dst_as as u32)
                .src_mask(src_mask)
                .dst_mask(dst_mask);

            v5header = v5header_builder.build().unwrap();
            v5record = v5record_builder.build().unwrap();

            nfmsg_builder
                .datetime(datetime.to_string())
                .exporter_addr(exporter_addr)
                .nfv5_header(v5header)
                .nfv5_record(v5record);

            nfmsgs.push(nfmsg_builder.build().unwrap());
        }
        Ok(nfmsgs)
    }
}
