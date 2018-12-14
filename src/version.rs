
use std::convert::TryFrom;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Version {
    V1,
    V2c,
    V2p,
    V2u,
    V3,
}

impl Into<u8> for Version {
    fn into(self) -> u8 {
        use self::Version::*;

        match self {
            V1        => 0x00,
            V2c       => 0x01,
            V2p | V2u => 0x02,
            V3        => 0x03,
        }
    }
}

impl TryFrom<u8> for Version {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use self::Version::*;

        match value {
            0 => Ok(V1),
            1 => Ok(V2c),
            2 => Ok(V2p),  // V2p & V2u
            3 => Ok(V3),
            _ => Err(()),
        }
    }
}