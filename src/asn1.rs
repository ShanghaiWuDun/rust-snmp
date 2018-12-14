use crate::der::DerEncoder;

use std::fmt;
use std::hash;
use std::io::{ self, Write, Read, };
use std::ops::BitOr;


#[repr(u8)]
#[derive(Debug)]
pub enum Class {
    Universal       = 0b00000000u8,
    Application     = 0b01000000u8,
    ContextSpecific = 0b10000000u8,
    Private         = 0b11000000u8,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Kind {
    Primitive   = 0b00000000u8,
    Constructed = 0b00100000u8,
}


pub const AAAA: u8 = Kind::Primitive as u8 | Class::Universal as u8;


impl BitOr<Self> for Kind {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitOr<u8> for Kind {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        self as u8 | rhs
    }
}

impl BitOr<Class> for Kind {
    type Output = u8;

    fn bitor(self, rhs: Class) -> Self::Output {
        self as u8 | rhs as u8
    }
}



impl BitOr<Self> for Class {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitOr<u8> for Class {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        self as u8 | rhs
    }
}

impl BitOr<Kind> for Class {
    type Output = u8;

    fn bitor(self, rhs: Kind) -> Self::Output {
        self as u8 | rhs as u8
    }
}


pub trait Value: fmt::Debug + Clone + hash::Hash + PartialEq + Eq {
    fn id(&self) -> u8 {
        self.class() | self.kind() | self.tag()
    }
    fn class(&self) -> Class;
    fn kind(&self) -> Kind;
    fn tag(&self) -> u8;
    fn length(&self) -> usize;
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Boolean(bool);

/// capable of holding a value from -2,147,483,648 to +2,147,483,647
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Integer(i32);
pub type Integer32 = Integer;

// #[derive(Debug)]
// pub struct Real(f64);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OctetString(String);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Null;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ObjectIdentifier(Vec<u32>);


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Sequence {

}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Set {

}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct UTCTime(u32);


// impl<W: Write, V: Value> DerEncoder<W> for V {
//     fn der_encode(&mut self, output: W) {
//         unimplemented!()
//     }
// }

impl Value for Boolean {
    fn class(&self) -> Class {
        Class::Universal
    }

    fn kind(&self) -> Kind {
        Kind::Primitive
    }

    fn tag(&self) -> u8 {
        1
    }

    fn length(&self) -> usize {
        1
    }
}

impl Value for Integer {
    fn class(&self) -> Class {
        Class::Universal
    }

    fn kind(&self) -> Kind {
        Kind::Primitive
    }

    fn tag(&self) -> u8 {
        2
    }

    fn length(&self) -> usize {
        let bytes = self.0.to_be_bytes();
        let mut prefix = 0usize;
        
        for byte in bytes.iter() {
            if byte != &0u8 {
                break;
            }
            prefix += 1;
        }

        let len = 4 -  prefix;

        len
    }
}

impl Value for OctetString {
    fn class(&self) -> Class {
        Class::Universal
    }

    fn kind(&self) -> Kind {
        Kind::Primitive
    }

    fn tag(&self) -> u8 {
        4
    }

    fn length(&self) -> usize {
        self.0.as_bytes().len()
    }
}

impl Value for Null {
    fn class(&self) -> Class {
        Class::Universal
    }

    fn kind(&self) -> Kind {
        Kind::Primitive
    }

    fn tag(&self) -> u8 {
        5
    }

    fn length(&self) -> usize {
        // FIXME: 0 or 1 ?
        1
    }
}

impl Value for ObjectIdentifier {
    fn class(&self) -> Class {
        Class::Universal
    }

    fn kind(&self) -> Kind {
        Kind::Primitive
    }

    fn tag(&self) -> u8 {
        6
    }

    fn length(&self) -> usize {
        // FIXME: 需要确定该类型的编码方式。
        self.0.len()
    }
}


impl Value for Sequence {
    fn class(&self) -> Class {
        Class::Universal
    }

    fn kind(&self) -> Kind {
        Kind::Constructed
    }

    fn tag(&self) -> u8 {
        16
    }

    fn length(&self) -> usize {
        unimplemented!()
    }
}

impl Value for Set {
    fn class(&self) -> Class {
        Class::Universal
    }

    fn kind(&self) -> Kind {
        Kind::Constructed
    }

    fn tag(&self) -> u8 {
        17
    }

    fn length(&self) -> usize {
        unimplemented!()
    }
}


impl Value for UTCTime {
    fn class(&self) -> Class {
        Class::Universal
    }

    fn kind(&self) -> Kind {
        // Kind::Constructed
        unimplemented!()
    }

    fn tag(&self) -> u8 {
        23
    }

    fn length(&self) -> usize {
        unimplemented!()
    }
}


impl<W: Write> DerEncoder<W> for Boolean {
    fn der_encode(&mut self, mut output: &mut W) -> Result<(), io::Error> {
        self.der_encode_length(&mut output, self.length())?;

        output.write(&[self.0 as u8])?;

        Ok(())
    }
}

impl<W: Write> DerEncoder<W> for Integer {
    fn der_encode(&mut self, mut output: &mut W) -> Result<(), io::Error> {
        let len = self.length();
        self.der_encode_length(&mut output, len)?;

        let bytes = self.0.to_be_bytes();

        output.write(&bytes[bytes.len()-len..])?;

        Ok(())
    }
}

impl<W: Write> DerEncoder<W> for OctetString {
    fn der_encode(&mut self, mut output: &mut W) -> Result<(), io::Error> {
        let len = self.length();
        self.der_encode_length(&mut output, len)?;

        let bytes = self.0.as_bytes();

        output.write(&bytes[bytes.len()-len..])?;
        
        Ok(())
    }
}

impl<W: Write> DerEncoder<W> for Null {
    fn der_encode(&mut self, mut output: &mut W) -> Result<(), io::Error> {
        unimplemented!()
    }
}

impl<W: Write> DerEncoder<W> for ObjectIdentifier {
    fn der_encode(&mut self, mut output: &mut W) -> Result<(), io::Error> {
        unimplemented!()
    }
}

impl<W: Write> DerEncoder<W> for Sequence {
    fn der_encode(&mut self, mut output: &mut W) -> Result<(), io::Error> {
        unimplemented!()
    }
}

impl<W: Write> DerEncoder<W> for Set {
    fn der_encode(&mut self, mut output: &mut W) -> Result<(), io::Error> {
        unimplemented!()
    }
}

impl<W: Write> DerEncoder<W> for UTCTime {
    fn der_encode(&mut self, mut output: &mut W) -> Result<(), io::Error> {
        unimplemented!()
    }
}






impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl fmt::Display for OctetString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Null")
    }
}

impl fmt::Display for ObjectIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.iter()
                      .map(|node| node.to_string())
                      .collect::<Vec<String>>()
                      .join(".");
        write!(f, "{}", s)
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

