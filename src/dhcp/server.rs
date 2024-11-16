use crate::dhcp::{packet::*, FLAG_ZERO};
use crate::dhcp::{DhcpLease, LeaseError,DhcpMessageTypeCode, DhcpOption, BOOTREPLY, BOOTREQUEST};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::{cell::Cell, fmt::Result, io::Error};

const LEASE_NUM: u32 = 252;

pub struct DhcpServer {
    leases: HashMap<IpAddr, DhcpLease>,
    last_leases: u32,
    default_lease_duration: u32,
    lease_num: u32,
    lease_start: IpAddr,

    server: Server,
    server_id: u32,
}

impl DhcpServer {
    fn get_lease_from_ip(&self, ip: &IpAddr) -> Option<&DhcpLease> {
        self.leases.get(ip)
    }
    fn get_lease_from_chi(&self, chi: &[u8]) -> Option<&DhcpLease> {
        for (_, lease) in self.leases.iter() {
            match lease.get_chi() {
                Some(c) => {
                    if c.as_bytes() == chi {
                        // match client identifier
                        return Some(lease);
                    }
                }
                _ => {
                    if lease.get_mac().get_octets() == chi {
                        // match mac address
                        return Some(lease);
                    }
                }
            }
        }
        None
    }
    fn get_pair_from_chi(&self, chi: &[u8]) -> Option<(&IpAddr, &DhcpLease)> {
        for (ipaddr, lease) in self.leases.iter() {
            match lease.get_chi() {
                Some(c) => {
                    if c.as_bytes() == chi {
                        // match client identifier
                        return Some((ipaddr,lease));
                    }
                }
                _ => {
                    if lease.get_mac().get_octets() == chi {
                        // match mac address
                        return Some((ipaddr,lease));
                    }
                }
            }
        }
        None
    }
    fn get_available_ip(&self) -> &IpAddr {
        let start = match self.lease_start {
            IpAddr::V4(ipv4) => u32::from_be_bytes(ipv4.octets()),
            IpAddr::V6(ipv6) => todo!(),
        };
        for _ in 0..self.lease_num {
            
        }
    }

    fn handle_dhcp_discover(&self, in_packet: &Packet) {
        // Check if leases for chaddr exist
        // according to chaddr (or client-identifier in options)
        // Get slice
        let chi: &[u8] = match in_packet.get_client_identifier() {
            Some(chi) => &chi[..], // TODO may need to parse &[u8]
            None => {
                // No client identifier in options
                in_packet.get_chaddr()
            }
        };
        #[cfg(debug_print)]
        println!("[DEBUG] In handle_dhcp_discover");
        // assume the length of client identifier described in option
        // is the same to mac address (= 6)
        let ip: &IpAddr = if let Some(lease) = self.get_lease_from_chi(chi) {
            // lease match
            #[cfg(debug_print)]
            println!("[DEBUG] Lease found: {:?} for chi:{:?}", lease, chi);
            lease.get_ip()

        } else {
            #[cfg(debug_print)]
            println!("[DEBUG] No Lease found for chi:{:?}", chi);
            self.get_available_ip()
        };
        let ip: u32 = match ip {
            IpAddr::V4(ipv4) => Some(u32::from_be_bytes(ipv4.octets())),
            IpAddr::V6(ipv6) => None,
        }.unwrap();
        #[cfg(debug_print)]
        println!("[DEBUG] Send reply");
        let pre_packet: Packet = Packet::new(
            BOOTREPLY,
            1,
            6,
            0,
            in_packet.get_xid().try_into().expect("Failed to convert xid"),
            0,
            FLAG_ZERO,
            0, // 0 (DHCPDISCOVER), client's network address (DHCPINFORM)
            ip,
            0,
            0,
            in_packet.get_chaddr().try_into().expect("Failed to convert chaddr"),
            in_packet.get_sname().try_into().expect("Failed to convert sname"),
            in_packet.get_file().try_into().expect("Failed to convert file"),
            vec![
                DhcpOption::ServerIdentifier(self.server.server_ip),
                DhcpOption::DhcpMessageType(DhcpMessageTypeCode::Offer),
            ]
        );
    }
    fn handle_dhcp_request(&self, in_packet: &Packet) {
        todo!()
    }
    fn handle_dhcp_decline(&self, in_packet: &Packet) {
        todo!()
    }
    fn handle_dhcp_release(&self, in_packet: &Packet) {
        todo!()
    }
    

}
impl Handler for DhcpServer {
    fn handle_request(&mut self, in_packet: &Packet) {
        if !self.server.is_for_this_server(in_packet) {
            // Check for this server
            return;
        }
        match in_packet.get_dhcp_message_type() {
            Some(DhcpMessageTypeCode::Discover) => self.handle_dhcp_discover(in_packet),
            Some(DhcpMessageTypeCode::Request) => self.handle_dhcp_request(in_packet),
            Some(DhcpMessageTypeCode::Decline) => self.handle_dhcp_decline(in_packet),
            Some(DhcpMessageTypeCode::Release) => self.handle_dhcp_release(in_packet),
            // Ack = 5,
            // Nak = 6,
            // Release = 7,
            Some(_) => todo!(),
            None => todo!(),
        }
    }
}

pub struct DhcpMonitor {
    server: Server,
}

impl Handler for DhcpMonitor {
    fn handle_request(&mut self, in_packet: &Packet) {
        // TODO
        // Just add log?
        todo!()
    }
}

pub struct Server {
    in_buf: Cell<[u8; 2048]>,
    out_buf: Cell<[u8; 2048]>,
    socket: UdpSocket,
    socket_src: Cell<SocketAddr>,
    server_ip: IpAddr,
    broadcast_ip: IpAddr,
    loopback_ip: IpAddr,
}

pub trait Handler {
    fn handle_request(&mut self, in_packet: &Packet);
}

impl Server {
    pub fn serve<H: Handler>(&mut self, handler: &mut H) -> Error {
        loop {
            let (len, recv_src) = match self.socket.recv_from(self.in_buf.get_mut()) {
                Ok(it) => it,
                Err(err) => return err,
            };
            if let Ok(p) = Packet::decode_from_unchecked(&self.in_buf.get()[..len]) {
                // if let Ok(p) = Packet::from_unchecked(&self.in_buf.get()[..len]) {
                self.socket_src.set(recv_src);
                // write the out_buf
                handler.handle_request(&p);
            }
            // let in_packet = Packet::from_bytes(&self.in_buf[..len]);
            // let out_packet = handler.handle_request(self, &in_packet);
            // if let Some(out_packet) = out_packet {
            //     self.socket.send_to(&out_packet.to_bytes(), self.socket_src).unwrap();
            //     handler.handle_reply(self, &out_packet);
        }
    }

    pub fn reply(
        &self,
        offer_ip: IpAddr,
        optins_vec: Vec<DhcpOption>,
        msg_type: DhcpMessageTypeCode,
        prepare_packet: Packet,
    ) -> std::io::Result<usize> {
        let mut send_packet = &prepare_packet;
        // send_packet.set_op(BOOTREPLY);
        // // Ethernet (100Mb/1Gb/2.5G etc)
        // send_packet.set_htype(1);
        // send_packet.set_hlen(6);
        // send_packet.set_hops(0);
        // send_packet.set_xid(prepare_packet.get_xid());
        // send_packet.set_secs(0);

        self.send_packet(send_packet)
    }

    pub fn is_for_this_server(&self, p: &Packet) -> bool {
        let ip = p.get_server_ip();
        ip == self.server_ip || ip.is_unspecified()
    }

    // send DHCP packet back
    pub fn send_packet(&self, send_p: &Packet) -> std::io::Result<usize> {
        let mut addr: SocketAddr = self.socket_src.get();
        // let addr = self.socket_src.get();
        // check broadcast
        // TODO
        // Add check for broadcast/loopback for send_p
        // e.g. if send_p.is_broadcast()
        if addr.ip().is_unspecified() {
            addr.set_ip(self.broadcast_ip);
        }
        // check local loopback
        if addr.ip().is_loopback() {
            addr.set_ip(self.loopback_ip);
        }
        self.socket
            .send_to(send_p.encode(&mut self.out_buf.get()), addr)
    }

    // pub fn reply
}
