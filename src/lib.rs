pub mod dhcp;
pub mod macaddress;

// write a test function in lib.rs and start test
#[cfg(test)]
mod tests {
    #[test]
    fn test_dhcp() {
        crate::dhcp::test_options();
    }
}