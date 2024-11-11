// Server configuration
use std::net::{IpAddr,Ipv4Addr,Ipv6Addr}
use std::fs::File


// All use ipv4 at present
const SERVER_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
const SUBNET_MASK: IpAddr = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0));

const SUNET_IP_RANGE: [IpAddr; 2] = [
    IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
    IpAddr::V4(Ipv4Addr::new(192, 168, 1, 254)),
];


const DNS_IPS: [IpAddr; 2] = [
    // Google DNS servers
    IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
    // CloudFlare DNS servers
    IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
    // TODO configure from toml or other files
];
const ROUTER_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
const BROADCAST_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 255));


// Lease time and number
const LEASE_DURATION_SECS: u32 = 86400;
const LEASE_NUM: u32 = 252;

// TODO: Derived constants
// const IP_START_NUM: u32 = u32::from_be_bytes(IP_START);
// const INFINITE_LEASE: Option<Instant> = None; // Special value for infinite lease

fn main() {
  let socket =UdpSocket::bind()
}