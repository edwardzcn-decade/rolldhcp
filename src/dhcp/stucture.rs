// Use `Ipv4Addr` and `Ipv6Addr` from `std::net` module
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;


// Define DHCP option type (according to RFC 2131)
// The 'option' field is now variable length. A DHCP client must be prepared to receive DHCP messages with an 'options' field of at least length 312 octets. That's 548 at least in total.
pub enum DhcpOptions {
  // TODO
}

// Define DHCP message structure (according to RFC 951, 2131)
// DHCP uses the BOOTP message format defined in RFC 951

