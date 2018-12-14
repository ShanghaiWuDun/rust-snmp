#![feature(try_from, duration_as_u128, const_fn)]
#![allow(unused_mut, unused_variables, unused_imports, dead_code)]


use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use std::io::Write;

pub mod asn1;
pub mod der;
pub mod v1;
pub mod v3;
pub mod error;
pub mod version;
pub mod smi;

#[allow(non_snake_case)]
pub mod pen;
#[allow(non_snake_case)]
pub mod mgmt;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Message<V: asn1::Value> {
    version: version::Version,
    community: Vec<u8>,
    pdu: v1::Pdu<V>,
}


pub trait MessageTrait<W: Write>: der::DerEncoder<W> {
    type Packet;

    fn version(&self) -> version::Version;
    fn packet(&self) -> Self::Packet;
}


fn main() {
    println!("It works!");
}
