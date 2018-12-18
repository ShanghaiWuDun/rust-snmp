
// SNMPv1 PDUs: GetRequest-PDU, GetNextRequest-PDU, SetRequest-PDU and GetResponse-PDU:
use crate::version::Version;
use crate::error::ErrorStatus;
use crate::der::{ self, Encoder, };
use crate::asn1::{ self, Value, ValueKind, };


use std::fmt;
use std::convert::TryFrom;
use std::io::{ self, Read, Write, };



#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PduKind {
    // SNMP V1
    GetRequest,
    GetNextRequest,
    GetResponse,
    SetRequest,
    // WARN: 在 SNMP V2C 当中，TrapV1 已被 TrapV2 替代。
    // http://www.tcpipguide.com/free/t_SNMPVersion1SNMPv1MessageFormat-3.htm
    TrapV1,
    // SNMP V2C
    GetBulkRequest,
    InformRequest,
    TrapV2,
    Report,
}


pub trait Message<W: Write + fmt::Debug>: fmt::Debug {
    fn version(&self) -> Version;
}


impl<W: Write + fmt::Debug> Message<W> for V1Message<W> {
    fn version(&self) -> Version {
        Version::V2c
    }
}



#[derive(Debug)]
pub struct V1Message<W: Write + fmt::Debug> {
    community: String,
    pdu: Box<Pdu<W>>,
}


// SNMPv1 Trap-PDU Format
// http://www.tcpipguide.com/free/t_SNMPVersion1SNMPv1MessageFormat-3.htm
// SNMP Protocol Information Notification Using Trap(v2) and InformRequest Messages 
// http://www.tcpipguide.com/free/t_SNMPProtocolInformationNotificationUsingTrapv2andI.htm
// SNMPv2 InformRequest Message
// http://www.tcpipguide.com/free/t_SNMPProtocolInformationNotificationUsingTrapv2andI-2.htm
pub trait Pdu<W: Write + fmt::Debug>: fmt::Debug + Encoder<W> {

}

impl<W: Write + fmt::Debug> Value for V1Message<W> {
    fn kind(&self) -> ValueKind {
        // Sequence
        asn1::Sequence{ }.kind()
    }
}

impl<W: Write + fmt::Debug> Encoder<W> for V1Message<W> {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        // version  : Integer       4 Bytes
        // community: Octet String  Variable
        // pdu      : _             Variable
        unimplemented!()
    }
}




#[derive(Debug)]
pub struct Variables {
    pub key: asn1::ObjectIdentifier,
    pub val: Box<Value>,
}

#[derive(Debug)]
pub struct GetRequest {
    request_id: i32,
    error_status: ErrorStatus,
    error_index: i32,
    variable_bindings: Vec<(asn1::ObjectIdentifier, Box<Value>)>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GetNextRequest {

}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GetResponse {
    request_id: i32,
    error_status: ErrorStatus,
    error_index: i32,
    variable_bindings: Vec<u8>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SetRequest {

}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TrapV1 {

}



#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GetBulkRequest {

}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InformRequest {

}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TrapV2 {

}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Report {

}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NoSuchObject {

}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NoSuchInstance {

}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EndOfMibView {

}

asn_value!{ GetRequest, 160 }
asn_value!{ GetNextRequest, 161 }
asn_value!{ GetResponse, 162 }
asn_value!{ SetRequest, 163 }
asn_value!{ TrapV1, 164 }
asn_value!{ GetBulkRequest, 165 }
asn_value!{ InformRequest, 166 }
asn_value!{ TrapV2, 167 }
asn_value!{ Report, 168 }

asn_value!{ NoSuchObject, 128 }
asn_value!{ NoSuchInstance, 129 }
asn_value!{ EndOfMibView, 130 }


impl<W: Write + fmt::Debug> Encoder<W> for GetRequest {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for GetRequest { }


impl<W: Write + fmt::Debug> Encoder<W> for GetNextRequest {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for GetNextRequest { }


impl<W: Write> Encoder<W> for GetResponse {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for GetResponse { }


impl<W: Write + fmt::Debug> Encoder<W> for SetRequest {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for SetRequest { }


impl<W: Write + fmt::Debug> Encoder<W> for TrapV1 {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for TrapV1 { }


impl<W: Write> Encoder<W> for GetBulkRequest {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for GetBulkRequest { }


impl<W: Write + fmt::Debug> Encoder<W> for InformRequest {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for InformRequest { }


impl<W: Write> Encoder<W> for TrapV2 {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for TrapV2 { }

impl<W: Write> Encoder<W> for Report {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for Report { }


impl<W: Write + fmt::Debug> Encoder<W> for NoSuchObject {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for NoSuchObject { }

impl<W: Write> Encoder<W> for NoSuchInstance {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for NoSuchInstance { }

impl<W: Write + fmt::Debug> Encoder<W> for EndOfMibView {
    fn encode(&mut self, output: &mut W) -> Result<usize, io::Error> {
        let type_id = self.kind().type_id();
        unimplemented!();
        // let payload = &[0u8, 1u8, 2u8,];
        // der::encode(type_id, payload, output)
    }
}

impl<W: Write + fmt::Debug> Pdu<W> for EndOfMibView { }

