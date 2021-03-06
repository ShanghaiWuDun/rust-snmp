
use crate::asn1::Value;
use crate::asn1::ValueKind;
use crate::asn1::Boolean;

use std::fmt;
use std::io::{ self, Read, Write, };


// https://docs.microsoft.com/zh-cn/windows/desktop/SecCertEnroll/about-encoded-length-and-value-bytes
const USIZE_LEN: usize = std::mem::size_of::<usize>();


fn encode_length<W: Write>(length: usize, output: &mut W) -> Result<usize, io::Error> {
    if length < 128 {
        output.write_all(&[length as u8])
            .map(|_| 1)
    } else {
        // NOTE: TLV MAX Length is 256**126
        const AMT: usize = USIZE_LEN + 1;

        output.write_all(&[128u8 | USIZE_LEN as u8])?;
        output.write_all(&length.to_be_bytes())
            .map(|_| AMT)
    }
}

fn decode_length<R: Read>(input: &mut R) -> Result<(usize, usize), io::Error> {
    let first_byte = {
        let mut buffer = [0u8; 1];
        input.read_exact(&mut buffer)?;
        buffer[0]
    };
    let mut readed = 1usize;
    
    if first_byte == 255 {
        // reserved for future use
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Ooops ..."));
    }

    if first_byte < 128 {
        let length = first_byte as usize;
        Ok((length, readed))
    } else {
        let len = (first_byte & 127) as usize;
        
        if len == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Ooops ..."));
        }

        if len > USIZE_LEN {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Ooops ..."));
        }

        let mut length_bytes = [0u8; USIZE_LEN];

        input.read_exact(&mut length_bytes[(USIZE_LEN - len)..])?;
        readed += len;

        let length = usize::from_be_bytes(length_bytes);

        Ok((length, readed))
    }
}


pub fn encode<W: Write>(type_id: u8, payload: &[u8], output: &mut W) -> Result<usize, io::Error> {
    output.write_all(&[type_id])?;

    let mut size = 1usize;

    size += encode_length(payload.len(), output)?;

    output.write_all(payload)?;
    size += payload.len();

    Ok(size)
}



pub fn decode<R, F, V>(input: &mut R, mut handle: F) -> Result<(V, usize), io::Error> 
where
    R: Read,
    V: Value,
    F: FnMut(ValueKind, usize, &mut R) -> Result<(V, usize), io::Error> {

    let mut readed = 0usize;

    let mut type_id = [0u8; 1];
    input.read_exact(&mut type_id)?;
    let type_id = type_id[0];
    readed += 1;

    let (payload_length, amt) = decode_length(input)?;
    readed += amt;

    let (v, amt): (V, usize) = handle(ValueKind(type_id), payload_length, input)?;
    readed += amt;

    Ok((v, readed))

    // let mut payload = vec![0u8; payload_length];

    // input.read_exact(&mut payload)?;
    // readed += payload_length;

    // Ok(DerObject {
    //     id: type_id,
    //     payload: payload,
    //     size: readed,
    // })
}



pub trait Encoder<W: Write>: Value {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error>;
}

pub trait Decoder<R: Read>: Value + Sized {
    fn decode(input: &mut R) -> Result<Self, io::Error>;
}



#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_encode_length() {
        let mut output = io::Cursor::new(vec![]);
        
        for n in 0u8..128u8 {
            let res = encode_length(n as usize, &mut output);
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap(), 1usize);

            let mut data = output.get_mut();
            assert_eq!(data, &[n]);
            
            data.clear();
            output.set_position(0);
        }
        
        {
            let res = encode_length(128usize, &mut output);
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap(), USIZE_LEN + 1);

            let mut data = output.get_mut();
            assert_eq!((data[0] & 127) as usize, USIZE_LEN);

            let mut usize_bytes = [0u8; USIZE_LEN];
            usize_bytes.copy_from_slice(&data[1..]);

            assert_eq!(usize::from_be_bytes(usize_bytes), 128usize);
            
            data.clear();
            output.set_position(0);
        }

        {
            let res = encode_length(4394usize, &mut output);
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap(), USIZE_LEN + 1);

            let mut data = output.get_mut();
            assert_eq!((data[0] & 127) as usize, USIZE_LEN);

            let mut usize_bytes = [0u8; USIZE_LEN];
            usize_bytes.copy_from_slice(&data[1..]);
            
            assert_eq!(usize::from_be_bytes(usize_bytes), 4394usize);
            
            data.clear();
            output.set_position(0);
        }

    }

    #[test]
    fn test_encode() {
        let mut output = io::Cursor::new(vec![]);
        let res = encode(16u8, &[1u8, 2, 3, 4, 5, 6], &mut output);
        assert_eq!(res.is_ok(), true);
        let mut data = output.get_mut();
        assert_eq!(data, &[16u8, 6, 1u8, 2, 3, 4, 5, 6]);

        data.clear();
        output.set_position(0);


        let payload = (0u8..127).collect::<Vec<u8>>();
        let res = encode(16u8, &payload, &mut output);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 129);
        let mut data = output.get_mut();
        assert_eq!(&data[0..2], &[16u8, 127]);
        assert_eq!(&data[2..], &payload[..]);

        data.clear();
        output.set_position(0);


        let payload = (0u8..128).collect::<Vec<u8>>();
        let res = encode(16u8, &payload, &mut output);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 128+2+USIZE_LEN);
        let mut data = output.get_mut();
        assert_eq!(&data[0..2], &[16u8, (128 | USIZE_LEN) as u8]);
        assert_eq!(&data[2..2+USIZE_LEN], &(128usize).to_be_bytes() );
        assert_eq!(&data[2+USIZE_LEN..], &payload[..]);

        data.clear();
        output.set_position(0);
    }

    #[test]
    fn test_decode_length() {
        {
            let mut input = io::Cursor::new(vec![0u8]);
            let res = decode_length(&mut input);
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap(), (0usize, 1usize));
        }
        
        {
            let mut input = io::Cursor::new(vec![127u8]);
            let res = decode_length(&mut input);
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap(), (127usize, 1usize));
        }

        {
            let mut input = io::Cursor::new(vec![136u8, 0, 0, 0, 0, 0, 0, 0, 128]);
            let res = decode_length(&mut input);
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap(), (128usize, 9usize));
        }

        {
            let mut input = io::Cursor::new(vec![136u8, 0, 0, 0, 0, 0, 0, 17, 42]);
            let res = decode_length(&mut input);
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap(), (4394usize, 9usize));
        }
        

        {
            let mut input = io::Cursor::new(vec![255, 0, 0, 0, 0, 0, 0, 0, 128]);
            let res = decode_length(&mut input);
            assert_eq!(res.is_err(), true);
        }
        {
            let mut input = io::Cursor::new(vec![]);
            let res = decode_length(&mut input);
            assert_eq!(res.is_err(), true);
        }
    }

    // #[test]
    // fn test_decode() {
    //     let mut input = io::Cursor::new(vec![
    //         16u8, 136, 0, 0, 0, 0, 0, 0, 0, 128, 
    //         0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 
    //         11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    //         21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    //         31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    //         41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    //         51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
    //         61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
    //         71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    //         81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
    //         91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
    //         101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
    //         111, 112, 113, 114, 115, 116, 117, 118, 119, 120,
    //         121, 122, 123, 124, 125, 126, 127]);
    //     let res = decode(&mut input);
        
    //     assert_eq!(res.is_ok(), true);

    //     let der_obj = res.unwrap();
    //     assert_eq!(der_obj.id, 16u8);
    //     assert_eq!(&der_obj.payload[..], &(0u8..128).collect::<Vec<u8>>()[..]);
    // }
}
