#![feature(try_from, duration_as_u128, const_fn, uniform_paths)]
#![allow(unused_mut, unused_variables, unused_imports, dead_code)]


use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use std::io::{ self, Read, Write, };

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


// pub trait MessageTrait<W: Write>: der::DerEncoder<W> {
//     type Packet;

//     fn version(&self) -> version::Version;
//     fn packet(&self) -> Self::Packet;
// }


fn main() {
    use asn1::Boolean;
    use asn1::Value;
    use der::Decoder;
    use der::Encoder;


    let mut output = io::Cursor::new(vec![]);
    let mut v = Boolean(true);
    let res = v.encode(&mut output);
    println!("ASN.1 Value: {:?}  DER Encoded({:?}): {:?}", v, res, output.get_ref());


    let mut input = io::Cursor::new(vec![1u8, 1, 0]);
    let res = der::decode(&mut input, |value_kind: asn1::ValueKind, payload_length: usize, input| {
        let mut payload = vec![0u8; payload_length];
        input.read_exact(&mut payload)?;

        // Decode ASN.1 Value
        if value_kind == Boolean::kind() {
            let v = if payload.len() == 0 || payload[0] == 0 { false } else { true };
            Ok((Boolean(v), payload_length))
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Ooops ..."))
        }
    });
    println!("DER Encoded: {:?}  ASN.1 Value: {:?}", input.get_ref(), res);
}
