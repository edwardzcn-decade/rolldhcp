// Server configuration
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

use rolldhcp::macaddress::MacAddress;
// Add hashmap
use std::collections::HashMap;
// use rolldhcp::dhcp::{options, packet, server};

// All use ipv4 at present
const IPV4_ONLY: bool = true;
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
    // Set the UDP server
    let socket = if IPV4_ONLY {
        UdpSocket::bind("0.0.0.0:67")
    } else {
        UdpSocket::bind("[::]:67")
    }
    .expect("Could not bind to address");
    socket.set_broadcast(true).expect("Could not set broadcast");

    // Set lease storage
    let mut leases: HashMap<Ipv4Addr, (MacAddress, Option<Instant>)>;
}
