
use std::fmt;
use std::convert::TryFrom;


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ErrorStatus {
    // SNMP V1
    /// The agent reports that no errors occurred during transmission.
    noError,
    /// The agent could not place the results of the requested SNMP 
    /// operation in a single SNMP message.
    tooBig,
    /// The requested SNMP operation identified an unknown variable.
    noSuchName,
    /// The requested SNMP operation tried to change a variable but 
    /// it specified either a syntax or value error.
    badValue,
    /// The requested SNMP operation tried to change a variable that 
    /// was not allowed to change, according to the community 
    /// profile of the variable.
    readOnly,
    /// An error other than one of those listed here occurred during 
    /// the requested SNMP operation.
    genErr,

    // SNMP V2
    /// The specified SNMP variable is not accessible.
    noAccess,
    /// The value specifies a type that is inconsistent 
    /// with the type required for the variable.
    wrongType,
    /// The value specifies a length that is inconsistent 
    /// with the length required for the variable.
    wrongLength,
    /// The value contains an Abstract Syntax Notation 
    /// One (ASN.1) encoding that is inconsistent with 
    /// the ASN.1 tag of the field.
    wrongEncoding,
    /// The value cannot be assigned to the variable.
    wrongValue,
    /// The variable does not exist, and the agent cannot create it.
    noCreation,
    /// The value is inconsistent with values of other managed objects.
    inconsistentValue,
    /// Assigning the value to the variable requires allocation of 
    /// resources that are currently unavailable.
    resourceUnavailable,
    /// No validation errors occurred, but no variables were updated.
    commitFailed,
    /// No validation errors occurred. Some variables were updated 
    /// because it was not possible to undo their assignment.
    undoFailed,
    /// An authorization error occurred.
    authorizationError,
    /// The variable exists but the agent cannot modify it.
    notWritable,
    /// The variable does not exist; the agent cannot create it because 
    /// the named object instance is inconsistent with the values of 
    /// other managed objects.
    inconsistentName,
}

impl Into<u8> for ErrorStatus {
    fn into(self) -> u8 {
        use self::ErrorStatus::*;

        match self {
            noError => 0,
            tooBig => 1,
            noSuchName => 2,
            badValue => 3,
            readOnly => 4,
            genErr => 5,
            noAccess => 6,
            wrongType => 7,
            wrongLength => 8,
            wrongEncoding => 9,
            wrongValue => 10,
            noCreation => 11,
            inconsistentValue => 12,
            resourceUnavailable => 13,
            commitFailed => 14,
            undoFailed => 15,
            authorizationError => 16,
            notWritable => 17,
            inconsistentName => 18,
        }
    }
}

impl TryFrom<u8> for ErrorStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use self::ErrorStatus::*;

        match value {
            0 => Ok(noError),
            1 => Ok(tooBig),
            2 => Ok(noSuchName),
            3 => Ok(badValue),
            4 => Ok(readOnly),
            5 => Ok(genErr),
            6 => Ok(noAccess),
            7 => Ok(wrongType),
            8 => Ok(wrongLength),
            9 => Ok(wrongEncoding),
            10 => Ok(wrongValue),
            11 => Ok(noCreation),
            12 => Ok(inconsistentValue),
            13 => Ok(resourceUnavailable),
            14 => Ok(commitFailed),
            15 => Ok(undoFailed),
            16 => Ok(authorizationError),
            17 => Ok(notWritable),
            18 => Ok(inconsistentName),
            _ => Err(()),
        }
    }
}
