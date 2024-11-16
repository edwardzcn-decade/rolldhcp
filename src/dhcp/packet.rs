use crate::dhcp::*;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub enum GetMessageTypeError {
    NoMessageTypeOption,
    WrongCode,
}

#[derive(Debug)]
pub enum ConvertPacketError<T> {
    ParseFromBytesError(T),
    NonUtf8String,
    UnrecognizedMessageType,
    InvalidHlen,
}

trait FromBytes {
    fn from_be_bytes(a: &[u8]) -> Self;
}
// impl<const N: usize> FromBytes for [u8; N] {
//     fn from_be_bytes(a: &[u8]) -> Self {
//         let (int_bytes, rest) = a.split_at(N);
//         let mut me = [0u8; N];
//         me.copy_from_slice(int_bytes);
//         *a = rest;
//         me
//     }
// }
impl FromBytes for u64 {
    fn from_be_bytes(a: &[u8]) -> u64 {
        let array: [u8; 8] = a.try_into().expect("slice with incorrect length");
        u64::from_be_bytes(array)
    }
}
impl FromBytes for u32 {
    fn from_be_bytes(a: &[u8]) -> u32 {
        let array: [u8; 4] = a.try_into().expect("slice with incorrect length");
        u32::from_be_bytes(array)
    }
}
impl FromBytes for u16 {
    fn from_be_bytes(a: &[u8]) -> Self {
        let array: [u8; 2] = a.try_into().expect("slice with incorrect length");
        u16::from_be_bytes(array)
    }
}
impl FromBytes for u8 {
    fn from_be_bytes(a: &[u8]) -> Self {
        a[0]
    }
}

type ConvertPairResult<I, O> = Result<(I, O), ConvertPacketError<I>>;
type ConvertSingleResult<T> = Result<T, ConvertPacketError<T>>;

/// A DHCP Message structure described in [RFC 2131](https://datatracker.ietf.org/doc/html/rfc2131).
///
/// This structure is used to represent a DHCP message. DHCP uses the BOOTP
/// message format defined in [RFC 951](https://datatracker.ietf.org/doc/html/rfc951).
///
/// In 'op' filed
/// BOOTREQUEST: the DHCP message sent from a client to a server
/// BOOTREPLY: the DHCP message sent from a server to a client
///
///
/// The first four octets of the 'options' field of the DHCP message contain the
/// values 99, 130, 83, and 99 in that order (this is the same magic cookie in
/// [RFC 1497](https://datatracker.ietf.org/doc/html/rfc1497)).
/// # Examples
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Packet {
    op: u8,
    htype: u8,
    hlen: u8,
    hops: u8,
    xid: u32,
    secs: u16,
    flags: u16,
    ciaddr: u32, //client identifier which can be opaque to the servers
    yiaddr: u32,
    siaddr: u32, //server identifier: appoint the server to use in the next step in the client bootstrap process
    giaddr: u32,
    chaddr: [u8; 16],
    sname: [u8; 64],
    file: [u8; 128],
    options: Vec<DhcpOption>,
    // broadcast: bool,
    // loopback: bool,
}
fn custom_take<'a>(n: usize) -> impl Fn(&'a [u8]) -> ConvertPairResult<&'a [u8], &'a [u8]> {
    move |input: &'a [u8]| {
        if input.len() >= n {
            Ok((&input[n..], &input[0..n]))
        } else {
            Err(ConvertPacketError::InvalidHlen)
        }
    }
}
impl Packet {
    fn decode_with_offset<T: FromBytes>(
        input: &[u8],
        offset: usize,
        n: usize,
    ) -> ConvertSingleResult<T> {
        if input.len() < offset {
            return Err(ConvertPacketError::InvalidHlen);
        } else {
            Ok(T::from_be_bytes(&input[offset..offset + n]))
        }
    }
    fn decode(input: &[u8]) -> ConvertSingleResult<Packet> {
        // let op = decode_op_with_offset(input, 0)?;
        // let htype = decode_htype_with_offset(input, 1)?;
        // let hlen = decode_hlen_with_offset(input, 2)?;
        // let hops = decode_hops_with_offset(input, 3)?;
        // let xid = decode_xid_with_offset(input, 4)?;
        // let secs = decode_secs_with_offset(input, 8)?;
        // let flags = decode_flags_with_offset(input, 10)?;
        // let ciaddr = decode_ciaddr_with_offset(input, 12)?;
        // let yiaddr = decode_yiaddr_with_offset(input, 16)?;
        // let siaddr = decode_siaddr_with_offset(input, 20)?;
        // let giaddr = decode_giaddr_with_offset(input, 24)?;

        // TODO
        // decode to new slice
        // let chaddr = decode_chaddr_with_offset(input, 28)?;
        // let sname = decode_sname_with_offset(input, 44)?;
        // let file = decode_file_with_offset(input, 108)?;

        // decode DHCP options
        // let options = decode_options_with_offset(input, 236)?;

        // Try poly func
        let op = Self::decode_with_offset::<u8>(input, 0, 1).unwrap();
        let htype = Self::decode_with_offset::<u8>(input, 1, 1).unwrap();
        let hlen = Self::decode_with_offset::<u8>(input, 2, 1).unwrap();
        let hops = Self::decode_with_offset::<u8>(input, 3, 1).unwrap();
        let xid = Self::decode_with_offset::<u32>(input, 4, 4).unwrap();
        let secs = Self::decode_with_offset::<u16>(input, 8, 2).unwrap();
        let flags = Self::decode_with_offset::<u16>(input, 10, 2).unwrap();
        // let ciaddr = Packet::decode_with_offset::
        // let yiaddr = Packet::decode_with_offset::
        // let siaddr = Packet::decode_with_offset::
        // let giaddr = Packet::decode_with_offset::

        let ciaddr = todo!();
        let yiaddr = todo!();
        let siaddr = todo!();
        let giaddr = todo!();
        let chaddr = todo!();
        let sname = todo!();
        let file = todo!();
        let options = todo!();

        // skil the end tag byte
        // let end_tag = decode_end_tag_with_offset(input, 236 + options.len())?;

        Ok(Packet {
            op,
            htype,
            hlen,
            hops,
            xid,
            secs,
            flags,
            ciaddr,
            yiaddr,
            siaddr,
            giaddr,
            chaddr,
            sname,
            file,
            options,
            // broadcast: false,
            // loopback: false,
        })
    }

    // fn decode_op(input: &[u8], offset: usize) -> ConvertResult<u8> {
    //     todo!()
    // }
    // fn decode_flags(input: &[u8], offset: usize) -> ConvertResult<u16>> {
    //   todo!()
    // }
    fn option(&self, code: u8) -> Option<&DhcpOption> {
        self.options.iter().find(|&opt| opt.code() == code)
    }

    
}

impl Packet {
    pub fn new(
        op: u8,
        htype: u8,
        hlen: u8,
        hops: u8,
        xid: u32,
        secs: u16,
        flags: u16,
        ciaddr: u32,
        yiaddr: u32,
        siaddr: u32,
        giaddr: u32,
        chaddr: [u8; 16],
        sname: [u8; 64],
        file: [u8; 128],
        options: Vec<DhcpOption>,
    ) -> Packet {
        Packet {
            op,
            htype,
            hlen,
            hops,
            xid,
            secs,
            flags,
            ciaddr,
            yiaddr,
            siaddr,
            giaddr,
            chaddr,
            sname,
            file,
            options,
        }
    }
    pub fn encode(&self, buf: &mut [u8; 2048]) -> &[u8] {
        todo!()
    }
    pub fn decode_from_unchecked(bytes: &[u8]) -> Result<Packet, ConvertPacketError<Packet>> {
        // Check the length of bytes
        // RFC 2131
        // debugonly
        #[cfg(debug_assertions)]
        {
            println!("Packet bytes.len() = {}", bytes.len());
            assert!(bytes.len() <= 576);
        }
        Self::decode(bytes);
        todo!()
    }

    // Inside the Packet
    pub fn get_op(&self) -> u8 {
        self.op
    }
    pub fn get_htype(&self) -> u8 {
        self.htype
    }
    pub fn get_hlen(&self) -> u8 {
        self.hlen
    }
    pub fn get_hops(&self) -> u8 {
        self.hops
    }
    pub fn get_xid(&self) -> u32 {
        self.xid
    }
    pub fn get_secs(&self) -> u16 {
        self.secs
    }
    pub fn get_flags(&self) -> u16 {
        self.flags
    }
    pub fn get_ciaddr(&self) -> u32 {
        self.ciaddr
    }
    pub fn get_yiaddr(&self) -> u32 {
        self.yiaddr
    }
    pub fn get_siaddr(&self) -> u32 {
        self.siaddr
    }
    pub fn get_giaddr(&self) -> u32 {
        self.giaddr
    }
    pub fn get_chaddr(&self) -> &[u8] {
        &self.chaddr
    }
    pub fn get_sname(&self) -> &[u8] {
        &self.sname
    }
    pub fn get_file(&self) -> &[u8] {
        &self.file
    }
    pub fn get_options(&self) -> &[DhcpOption] {
        &self.options
    }

    pub fn get_server_ip(&self) -> IpAddr {
        // how to check ipv6
        IpAddr::V4(Ipv4Addr::from(self.siaddr))
    }
    pub fn get_dhcp_message_type(&self) -> Option<&DhcpMessageTypeCode> {
        // Check from options field in Packet
        match self.option(DHCP_MESSAGE_TYPE) {
            Some(DhcpOption::DhcpMessageType(msg_type_code)) => Some(msg_type_code),
            // Some(other_option) => Err(GetMessageTypeError::WrongCode),
            // _ => Err(GetMessageTypeError::NoMessageTypeOption)
            _ => None,
        }
    }
    pub fn get_client_identifier(&self) -> Option<&Vec<u8>> {
        // Check from options field in Packet
        match self.option(CLIENT_IDENTIFIER) {
            Some(DhcpOption::ClientIdentifier(id)) => Some(id),
            _ => None,
        }
    }
}

// Automatically transform from u8 slice
// only used after check
impl From<&[u8]> for Packet {
    #[inline]
    fn from(bytes: &[u8]) -> Packet {
        todo!()
    }
}

impl From<[u8; 2048]> for Packet {
    #[inline]
    fn from(mac: [u8; 2048]) -> Packet {
        todo!()
    }
}
