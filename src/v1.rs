

// SNMPv1 PDUs: GetRequest-PDU, GetNextRequest-PDU, SetRequest-PDU and GetResponse-PDU:

use crate::error::ErrorStatus;
use crate::asn1::{ self, Value, };


use std::io::{ self, Read, Write, };
use std::convert::TryFrom;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct V1Packet<V: Value> {
    version: u8,        // Integer       4 Bytes
    community: String,  // Octet String  Variable
    pdu: Pdu<V>,        // _             Variable
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PduKind {
    // SNMP V1
    GetRequest,
    GetNextRequest,
    GetResponse,
    SetRequest,
    // WARN: 在 SNMP V2C 当中，TrapV1 已被 TrapV2 替代。
    TrapV1,
    // SNMP V2C
    GetBulkRequest,
    InformRequest,
    TrapV2,
    Report,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Pdu<V: Value> {
    kind: PduKind,
    request_id: i32,
    error_status: ErrorStatus,
    error_index: i32,
    variable_bindings: Vec<Result<(Box<V>, Box<V>), ErrorStatus>>,
}

// SNMPv1 Trap-PDU Format
// http://www.tcpipguide.com/free/t_SNMPVersion1SNMPv1MessageFormat-3.htm
// SNMP Protocol Information Notification Using Trap(v2) and InformRequest Messages 
// http://www.tcpipguide.com/free/t_SNMPProtocolInformationNotificationUsingTrapv2andI.htm
// SNMPv2 InformRequest Message
// http://www.tcpipguide.com/free/t_SNMPProtocolInformationNotificationUsingTrapv2andI-2.htm
impl<V: Value> Pdu<V> {
    pub fn kind(&self) -> PduKind {
        self.kind
    }

    pub fn request_id(&self) -> i32 {
        self.request_id
    }

    pub fn error_status(&self) -> ErrorStatus {
        self.error_status
    }

    pub fn error_index(&self) -> i32 {
        self.error_index
    }

    pub fn variable_bindings(&self) -> &Vec<Result<(Box<V>, Box<V>), ErrorStatus>> {
        unimplemented!()
    }

    pub fn set_kind(&mut self, kind: PduKind) {
        self.kind = kind
    }

    pub fn set_request_id(&mut self, request_id: i32) {
        self.request_id = request_id
    }

    pub fn set_error_status(&mut self, error_status: ErrorStatus) {
        self.error_status = error_status
    }

    pub fn set_error_index(&mut self, error_index: i32) {
        self.error_index = error_index
    }

    pub fn set_variable_bindings(&mut self, vbindings: Vec<Result<(Box<V>, Box<V>), ErrorStatus>>) {
        unimplemented!()
    }
}



impl Into<u8> for PduKind {
    fn into(self) -> u8 {
        use self::PduKind::*;

        match self {
            GetRequest  => 0,
            GetNextRequest => 1,
            GetResponse => 2,
            SetRequest  => 3,
            TrapV1 => 4,
            GetBulkRequest => 5,
            InformRequest => 6,
            TrapV2 => 7,
            Report => 8,
        }
    }
}

impl TryFrom<u8> for PduKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use self::PduKind::*;

        match value {
            0 => Ok(GetRequest),
            1 => Ok(GetNextRequest),
            2 => Ok(GetResponse),
            3 => Ok(SetRequest),
            4 => Ok(TrapV1),
            5 => Ok(GetBulkRequest),
            6 => Ok(InformRequest),
            7 => Ok(TrapV2),
            8 => Ok(Report),
            _ => Err(()),
        }
    }
}

