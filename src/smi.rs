
// SMIv1 & SMIv2
// http://www.tcpipguide.com/free/t_TCPIPMIBObjectsObjectCharacteristicsandObjectTypes-3.htm#Table_206
// 

use std::net::Ipv4Addr;


// An IP address, encoded as a 4-byte octet string.
pub enum NetworkAddress {
    V4(Ipv4Addr),
}
pub type IpAddress = NetworkAddress;


// A 32-bit unsigned integer, that begins at 0 and increases 
// up to 4,294,967,295, then wraps back to 0.
pub struct Counter(u32);
pub type Counter32 = Counter;


// A 32-bit unsigned integer, that may have a value from 0 
// to 4,294,967,295 and may increase or decrease, like a gauge. 
// A minimum and maximum value are associated with the gauge, 
// indicating its normal range.
pub struct Gauge(u32);
pub type Gauge32 = Gauge;


// A 32-bit unsigned integer that indicates the number of hundredths 
// of seconds since some arbitrary start date.
// Used for timestamping and to compute elapsed time.
pub struct TimeTicks(u32);


// Data using arbitrary ASN.1 syntax that is to be passed 
// between devices without being interpreted. 
// As in NFS's XDR, the term “opaque” means that the data 
// is treated like a “black box” whose internal details cannot be seen.
// pub struct Opaque;


// A counter like Counter32 but 64 bits wide, 
// allowing a value from 0 to 18,446,744,073,709,551,615.
pub struct Counter64(u64);
