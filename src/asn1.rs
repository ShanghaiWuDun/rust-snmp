use crate::der::{ self, Encoder, Decoder, };

use std::fmt;
use std::hash;
use std::io::{ self, Write, Read, };
use std::ops::BitOr;
use std::convert::TryFrom;


// https://godoc.org/github.com/soniah/gosnmp#pkg-constants
// https://www.obj-sys.com/asn1tutorial/node124.html

pub const PRIMITIVE:             u8 = 0b00000000;
pub const CONSTRUCTED:           u8 = 0b00100000;



#[repr(u8)]
#[derive(Debug)]
pub enum ValueClass {
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


pub trait Value: fmt::Debug + Clone + hash::Hash + PartialEq + Eq {
    fn kind() -> ValueKind;
}



#[macro_export]
macro_rules! asn_value {
    ( $ty:ty, $num:expr ) => {
        impl Value for $ty {
            fn kind() -> ValueKind {
                ValueKind($num)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Boolean(pub bool);

/// capable of holding a value from -2,147,483,648 to +2,147,483,647
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Integer(i32);
pub type Integer32 = Integer;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BitString(String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OctetString(String);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Null;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ObjectIdentifier(Vec<u32>);

// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub struct ObjectDescriptor(Vec<u32>);



#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Sequence {

}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Set {

}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct UTCTime(u32);

// #[derive(Debug)]
// pub struct Real(f64);

asn_value!{ Boolean, 1 }
asn_value!{ Integer, 2 }
// asn_value!{ BitString, 3 }
asn_value!{ OctetString, 4 }
asn_value!{ Null, 5 }
asn_value!{ ObjectIdentifier, 6 }
// asn_value!{ ObjectDescriptor, 7 }


impl<W: Write> Encoder<W> for Boolean {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = Self::kind().type_id();
        let payload = &[self.0 as u8];
        der::encode(type_id, payload, output)
    }
}


impl<W: Write> Encoder<W> for Integer {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = Self::kind().type_id();
        let payload = &self.0.to_be_bytes();
        der::encode(type_id, payload, output)
    }
}

impl<W: Write> Encoder<W> for OctetString {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = Self::kind().type_id();
        let payload = self.0.as_bytes();
        der::encode(type_id, payload, output)
    }
}

impl<W: Write> Encoder<W> for Null {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = Self::kind().type_id();
        let payload = &[];
        der::encode(type_id, payload, output)
    }
}

impl<W: Write> Encoder<W> for ObjectIdentifier {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = Self::kind().type_id();
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