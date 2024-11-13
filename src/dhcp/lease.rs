use crate::macaddress::MacAddress;
use std::error::Error;
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DhcpLease {
    expiry: Duration,
    mac: MacAddress,
    ip_v: String,
    // ipv: [char; 2],
    ip: IpAddr,
    hostname: Option<String>,
    client_id: Option<String>,
}
impl DhcpLease {
    pub fn get_ip(&self) -> &IpAddr {
        &self.ip
    }
    pub fn get_mac(&self) -> &MacAddress {
        &self.mac
    }
    pub fn get_expiry(&self) -> Duration {
        self.expiry
    }
    pub fn get_expiry_secs(&self) -> u64 {
        self.expiry.as_secs()
    }
    pub fn get_hostname(&self) -> Option<String> {
        todo!()
    }
    pub fn get_client_id(&self) -> Option<String> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseDhcpLeaseError {
    ParseExpiredTimeError,
    InvalidFieldsLength,
    // InvalidMacAddress(ParseMacAddressError),
    InvalidMacAddress,
    NoSpecificIpVersion,
    InvalidIpAddr,
}
impl fmt::Display for ParseDhcpLeaseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // fmt.write_str(self.description())
        match self {
            ParseDhcpLeaseError::ParseExpiredTimeError => write!(fmt, "Parse expired time error"),
            ParseDhcpLeaseError::InvalidFieldsLength => write!(fmt, "Invalid fields length"),
            // ParseDhcpLeaseError::InvalidMacAddress(e) => write!(fmt, "Invalid MAC address: {}", e),
            ParseDhcpLeaseError::InvalidMacAddress => write!(fmt, "Invalid MAC address"),
            ParseDhcpLeaseError::NoSpecificIpVersion => write!(fmt, "No specific IP version"),
            ParseDhcpLeaseError::InvalidIpAddr => write!(fmt, "Invalid IP address"),
        }
    }
}

impl Error for ParseDhcpLeaseError {}

impl FromStr for DhcpLease {
    type Err = ParseDhcpLeaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s.split_whitespace().collect::<Vec<&str>>();
        // At least encluding 1. Lease Expiry Time, 2. MAC Address, 3. IP Address
        if fields.len() < 3 {
            return Err(ParseDhcpLeaseError::InvalidFieldsLength);
        }
        let expiry_secs = fields[0]
            .parse::<u64>()
            .map_err(|_| ParseDhcpLeaseError::ParseExpiredTimeError)?;
        let expiry = Duration::from_secs(expiry_secs);
        let mac: MacAddress =
            MacAddress::from_str(fields[1]).map_err(|_| ParseDhcpLeaseError::InvalidMacAddress)?;
        let ip_v = fields[2].to_string();
        if ip_v != "ipv4" && ip_v != "ipv6" {
            return Err(ParseDhcpLeaseError::NoSpecificIpVersion);
        }
        let ip: IpAddr = fields[3]
            .parse()
            .map_err(|_| ParseDhcpLeaseError::InvalidIpAddr)?;

        Ok(DhcpLease {
            expiry,
            mac,
            ip_v,
            ip,
            hostname: None,
            client_id: None,
        })
    }
}
