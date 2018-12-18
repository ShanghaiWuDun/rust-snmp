use crate::der::{ self, Encoder, Decoder, };

use std::fmt;
use std::hash;
use std::io::{ self, Write, Read, };
use std::ops::BitOr;
use std::convert::TryFrom;


// https://godoc.org/github.com/soniah/gosnmp#pkg-constants
pub const PRIMITIVE:             u8 = 0b00000000;

// Structured Types
// https://www.obj-sys.com/asn1tutorial/node11.html
pub const CONSTRUCTED:           u8 = 0b00100000;



#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ValueClass {
    // Universal Tags
    // https://www.obj-sys.com/asn1tutorial/node124.html
    Universal       = 0b00000000u8,
    Application     = 0b01000000u8,
    ContextSpecific = 0b10000000u8,
    Private         = 0b11000000u8,
}

impl BitOr<Self> for ValueClass {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitOr<u8> for ValueClass {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        self as u8 | rhs
    }
}


impl std::convert::TryFrom<u8> for ValueClass {
    type Error = ();

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0b00000000u8 => Ok(ValueClass::Universal),
            0b01000000u8 => Ok(ValueClass::Application),
            0b10000000u8 => Ok(ValueClass::ContextSpecific),
            0b11000000u8 => Ok(ValueClass::Private),
            _ => Err(()),
        }
    }
}

impl std::convert::Into<u8> for ValueClass {
    fn into(self) -> u8 {
        self as  u8
    }
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ValueKind(pub u8);

impl ValueKind {
    pub fn type_id(&self) -> u8 {
        self.0
    }

    pub fn class(&self) -> ValueClass {
        // 2**2
        ValueClass::try_from(self.0 & 0b1100_0000).unwrap()
    }

    pub fn constructed(&self) -> bool {
        // 0 or 1
        self.0 & 0b0010_0000 == 1
    }

    pub fn tag(&self) -> u8 {
        // 2**5
        self.0 & 0b0001_1111
    }
}


pub trait Value: fmt::Debug {
    fn kind(&self) -> ValueKind;
}



#[macro_export]
macro_rules! asn_value {
    ( $ty:ty, $num:expr ) => {
        impl Value for $ty {
            fn kind(&self) -> ValueKind {
                ValueKind($num)
            }
        }
    };
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RawValue {
    pub kind: ValueKind,
    pub payload: Vec<u8>,
}


// list ASN.1 Application type

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Boolean(pub bool);

/// capable of holding a value from -2,147,483,648 to +2,147,483,647
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Integer(pub i32);
pub type Integer32 = Integer;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BitString(pub String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OctetString(pub String);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Null;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ObjectIdentifier(pub Vec<u32>);

// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub struct ObjectDescriptor(Vec<u32>);



#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Sequence {

}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Set {

}


// list ASN.1 Application type
// SMIv1 & SMIv2
// http://www.tcpipguide.com/free/t_TCPIPMIBObjectsObjectCharacteristicsandObjectTypes-3.htm#Table_206


/// An IP address, encoded as a 4-byte octet string.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NetworkAddress {
    V4(std::net::Ipv4Addr),
}
pub type IpAddress = NetworkAddress;

/// A 32-bit unsigned integer, that begins at 0 and increases 
/// up to 4,294,967,295, then wraps back to 0.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Counter(pub u32);
pub type Counter32 = Counter;


/// A 32-bit unsigned integer, that may have a value from 0 
/// to 4,294,967,295 and may increase or decrease, like a gauge. 
/// A minimum and maximum value are associated with the gauge, 
/// indicating its normal range.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Gauge(pub u32);
pub type Gauge32 = Gauge;
pub type Unsigned32 = Gauge;


/// A 32-bit unsigned integer that indicates the number of hundredths 
/// of seconds since some arbitrary start date.
/// Used for timestamping and to compute elapsed time.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TimeTicks(pub u32);


// Data using arbitrary ASN.1 syntax that is to be passed 
// between devices without being interpreted. 
// As in NFS's XDR, the term “opaque” means that the data 
// is treated like a “black box” whose internal details cannot be seen.
// pub struct Opaque;


/// A counter like Counter32 but 64 bits wide, 
/// allowing a value from 0 to 18,446,744,073,709,551,615.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Counter64(pub u64);




#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct UTCTime(pub u32);

// #[derive(Debug)]
// pub struct Real(f64);

impl Value for RawValue {
    fn kind(&self) -> ValueKind {
        self.kind
    }
}

asn_value!{ Boolean, 1 }
asn_value!{ Integer, 2 }
// asn_value!{ BitString, 3 }
asn_value!{ OctetString, 4 }
asn_value!{ Null, 5 }
asn_value!{ ObjectIdentifier, 6 }
// asn_value!{ ObjectDescriptor, 7 }
asn_value!{ Sequence, 48 }
asn_value!{ Set, 49 }


asn_value!{ NetworkAddress, 64 }
asn_value!{ Counter, 65 }
// UNSIGNED32 GAUGE32 = 66
asn_value!{ Gauge, 66 }
asn_value!{ TimeTicks, 67 }
// asn_value!{ Opaque, 68 }
asn_value!{ Counter64, 70 }





impl<W: Write> Encoder<W> for Boolean {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        let payload = &[self.0 as u8];
        der::encode(type_id, payload, output)
    }
}


impl<W: Write> Encoder<W> for Integer {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        let payload = &self.0.to_be_bytes();
        der::encode(type_id, payload, output)
    }
}

impl<W: Write> Encoder<W> for OctetString {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        let payload = self.0.as_bytes();
        der::encode(type_id, payload, output)
    }
}

impl<W: Write> Encoder<W> for Null {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        let payload = &[];
        der::encode(type_id, payload, output)
    }
}

impl<W: Write> Encoder<W> for ObjectIdentifier {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        let mut payload = vec![0u8; self.0.len() * 4];

        for elem in self.0.iter() {
            let bytes: [u8; 4] = elem.to_be_bytes();
            for byte in &bytes {
                payload.push(*byte);
            }
        }

        der::encode(type_id, &payload, output)
    }
}