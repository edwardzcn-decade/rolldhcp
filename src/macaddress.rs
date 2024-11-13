use std::error::Error;
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseMacAddressError {
    InvalidLength,
    InvalidCharacter,
    // TODO
    // ParseError(String),
}

impl fmt::Display for ParseMacAddressError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // fmt.write_str(self.description())
        match self {
            ParseMacAddressError::InvalidLength => write!(fmt, "Invalid length"),
            ParseMacAddressError::InvalidCharacter => write!(fmt, "Invalid character"),
            // ParseMacAddressError::ParseError(s) => write!(fmt, "Parse error: {}", s),
        }
    }
}

impl Error for ParseMacAddressError {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MacAddress {
    octets: [u8; 6],
}

impl MacAddress {
    #[must_use]
    #[inline]
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> MacAddress {
        MacAddress {
            octets: [a, b, c, d, e, f],
        }
    }

    // TODO need test convert <-> u64
    #[inline]
    pub const fn to_bits(self) -> u64 {
        let extended_octests: [u8; 8] = [
            self.octets[0],
            self.octets[1],
            self.octets[2],
            self.octets[3],
            self.octets[4],
            self.octets[5],
            0,
            0,
        ];
        u64::from_be_bytes(extended_octests)
    }

    // TODO need test convert <-> u64
    #[inline]
    pub const fn from_bits(bits: u64) -> MacAddress {
        let bytes: [u8; 8] = bits.to_be_bytes();
        MacAddress {
            octets: [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]],
        }
    }

    #[inline]
    pub const fn nil() -> MacAddress {
        MacAddress { octets: [0x00; 6] }
    }
    pub const fn broadcast() -> MacAddress {
        MacAddress { octets: [0xff; 6] }
    }
    pub fn is_nil(&self) -> bool {
        self.octets.iter().all(|&x| x == 0x00)
    }
    pub fn is_broadcast(&self) -> bool {
        self.octets.iter().all(|&x| x == 0xff)
    }

    fn parese_from_str(mac_s: &str) -> Result<MacAddress, ParseMacAddressError> {
        // todo!()
        let strs: Vec<&str> = mac_s.split(':').collect();
        if strs.len() != 6 {
            return Err(ParseMacAddressError::InvalidLength)
        }
        let bytes= strs
            .iter()
            .map(|&x| u8::from_str_radix(x, 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| ParseMacAddressError::InvalidCharacter)?; // capture the ParseIntError here
        Ok(MacAddress::new(bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]))
    }
}

impl From<[u8; 6]> for MacAddress {
    /// Convert from `[u8; 6]` to `MacAddress`.
    #[inline]
    fn from(mac: [u8; 6]) -> MacAddress {
        MacAddress { octets: mac }
    }
}

impl From<u64> for MacAddress {
    /// Uses [`MacAddress::from_bits`] to convert a `u64` to a `MacAddress`.
    #[inline]
    fn from(mac: u64) -> MacAddress {
        MacAddress::from_bits(mac)
    }
}

impl From<MacAddress> for u64 {
    /// Uses [`MacAddress::to_bits`] to convert a `MacAddress` to a `u64`.
    #[inline]
    fn from(mac: MacAddress) -> u64 {
        mac.to_bits()
    }
}

impl FromStr for MacAddress {
    /// Parse a string of the form `00:11:22:33:44:55` as a MAC address.
    type Err = ParseMacAddressError;
    #[inline]
    fn from_str(mac_s: &str) -> Result<Self, Self::Err> {
        Self::parese_from_str(mac_s)
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octets = self.octets;
        // Just like write for Ipv4Addr, we can avoid the allocation here.
        // If there are no alignment requirements, write the IP address directly to `f`.
        // Otherwise, write it to a local buffer and then use `f.pad`.
        if fmt.precision().is_none() && fmt.width().is_none() {
            write!(
                fmt,
                "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
                octets[0], octets[1], octets[2], octets[3], octets[4], octets[5]
            )
        } else {
            // Longest possible compare to MAcAddress8
            const LONGEST_MAC8_ADDR: &str = "FF-FF-FF-FF-FF-FF-FF-FF";
            const LONGEST_MAC8_LEN: usize = LONGEST_MAC8_ADDR.len();
            let mut buf = String::with_capacity(LONGEST_MAC8_LEN);
            write!(
                buf,
                "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
                octets[0], octets[1], octets[2], octets[3], octets[4], octets[5]
            )
            .unwrap();
            fmt.pad(buf.as_str())
        }
    }
}
