// Server configuration
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;

use rolldhcp::dhcp::DhcpLease;

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
const LEASE_FILE_PATH: &str = "rolldhcp.leases";
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
    let mut leases_vec: Vec<DhcpLease> = vec![];
    // let mut leases_lut: HashMap<IpAddr, (MacAddress, Option<u64>)>;

    // Learning
    // Dismasq leases

    // The dnsmasq.leases file consists of entries where each line represents a single DHCP lease with fields separated by spaces. The typical format for each line is as follows:
    // 1.	Lease Expiry Time: The number of seconds since the Unix epoch (January 1, 1970) when the lease expires.
    // 2.	MAC Address: The hardware address of the device that holds the lease.
    // 3.	IP Address: The IP address that is leased to the device.
    // 4.	Hostname: The name of the device, if known.
    // 5.	Client ID: Optionally used to identify the client separately from its hardware address. if known. The client-ID is used as the computer's unique-ID in preference to the MAC address, if it's available. Some DHCP clients provide it, and some don't. The ones that do normally derive it from the MAC address unless explicity configured, but it could be something like a serial number, which would protect a computer from losing its identify if **the network interface were replaced**.

    // Get the leases from persistent storage
    let lease_file = File::open(LEASE_FILE_PATH).expect("Could not open lease file");
    // Read the lease_file according to the format mentioned above
    let reader = BufReader::new(lease_file);
    for line in reader.lines() {
        if let Err(e) = line {
            eprintln!("Error reading lease file: {}", e);
            return;
        }
        let line = line.unwrap();
        let l = DhcpLease::from_str(&line).expect("Failed to parse lease");
        leases_vec.push(l);
    }
    let leases_lut: HashMap<IpAddr, DhcpLease> = leases_vec
        .into_iter()
        .map(|lease| (lease.get_ip().clone(), lease))
        .collect::<HashMap<IpAddr, DhcpLease>>();
    // move ownership to leases_lut
    // compile only in debug mode
    #[cfg(debug_print)]
    {
        println!("[DEBUG PRINT] Leases: {:?}", leases_lut);
    }
    let dhcp_lease_server = DhcpServer {
        leases: leases_lut,
        last_leases: 0,
    };
    // start udp server
    // dhcp_lease_server.start(socket);
    // loop serve
    server::Server::serve(socket, SERVER_IP, BROADCAST_IP, dhcp_lease_server);
}
