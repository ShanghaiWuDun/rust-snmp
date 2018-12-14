
use std::fmt;
use std::io::{ self, Read, Write, };


// https://docs.rs/der-parser/1.1.0/der_parser/enum.DerObjectContent.html


pub trait DerEncoder<W: Write> {
    fn der_encode_length(&mut self, out: &mut W, length: usize) -> Result<(), io::Error> {
        if length >= 128 {
            let n = {
                let mut i = length;
                let mut bytes = 1;

                while i > 255 {
                    bytes += 1;
                    i >>= 8;
                }

                bytes
            };

            (*out).write_all(&[0x80 | n])?;

            for i in (1 .. n + 1).rev() {
                (*out).write_all(&[(length >> ((i - 1) * 8)) as u8])?;
            }
        } else {
            (*out).write_all(&[length as u8])?;
        }

        Ok(())
    }

    fn der_encode(&mut self, out: &mut W) -> Result<(), io::Error>;
}

pub trait DerDecoder<R: Read> {
    fn der_decode(&mut self, input: R);
}


impl<R: Read> DerDecoder<R> for Vec<u8> {
    fn der_decode(&mut self, input: R) {
        unimplemented!()
    }
}

impl<R: Read> DerDecoder<R> for &Vec<u8> {
    fn der_decode(&mut self, input: R) {
        unimplemented!()
    }
}

impl<R: Read> DerDecoder<R> for &[u8] {
    fn der_decode(&mut self, input: R) {
        unimplemented!()
    }
}


