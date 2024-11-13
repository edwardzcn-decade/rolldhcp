pub mod dhcp;
pub mod macaddress;

// write a test function in lib.rs and start test
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::macaddress::MacAddress;

    #[test]
    fn test_dhcp() {
        crate::dhcp::test_options();
    }

    #[test]
    fn test_storage() {
        crate::dhcp::test_storage();
    }

    #[test]
    fn test_macaddress() {
        // Parse a MAC address from String
        let test_mac_str = "00:11:22:33:44:55";
        let test_mac = MacAddress::from_str(&test_mac_str).expect("Failed to parse MAC address");
        println!("Parsed MAC address: {}", test_mac);


        // Fail to parse from [u8; 8] directly
        // trait bound `MacAddress: From<[{integer}; 8]>` is not satisfied
        // let test_mac = MacAddress::from([0xf4, 0x5c, 0x19, 0xaf, 0x96, 0x8d, 0x00, 0x00]); 
        let test_mac = MacAddress::from([0xf4, 0x5c, 0x19, 0xaf, 0x96, 0x8d]);
        println!("Parsed MAC address: {}", test_mac);


        // TODO add u64 convert test
    }
}
