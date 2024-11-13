use std::net::Ipv4Addr;
// One particular option
// the "DHCP message type" option - must be included in every DHCP
// message.  This option defines the "type" of the DHCP message.
// Additional options may be allowed, required, or not allowed,
// depending on the DHCP message type.

// 9.4. DHCP Message Type

//    This option is used to convey the type of the DHCP message.  The code
//    for this option is 53, and its length is 1.  Legal values for this
//    option are:

//            Value   Message Type
//            -----   ------------
//              1     DHCPDISCOVER
//              2     DHCPOFFER
//              3     DHCPREQUEST
//              4     DHCPDECLINE
//              5     DHCPACK
//              6     DHCPNAK
//              7     DHCPRELEASE
// Code   Len  Type
// +-----+-----+-----+
// |  53 |  1  | 1-7 |
// +-----+-----+-----+

pub enum DhcpOptions{

  // RFC 1497 Vendor Extensions
  // code 1 to 18 and code 255
  PadOption, // code 0
  EndOption, // code 255
  SubnetMask(Ipv4Addr), // code 1 If both the subnet mask and the router option are specified in a DHCP reply, the subnet mask option MUST be first.
  TimeOffset(u32), // code 2 The client's subnet in seconds from Coordinated Universal Time (UTC)

  // All of XServers should be listed in order of preference.
  // The minimum length is 4 octets, and the length MUST always be a multiple of 4.
  Routers(Vec<Ipv4Addr>), // code 3 // list of IP address
  TimeServers(Vec<Ipv4Addr>), // code 4
  NameServers(Vec<Ipv4Addr>), // code 5
  DomainNameServers(Vec<Ipv4Addr>), // code 6
  LogServers(Vec<Ipv4Addr>), // code 7
  CookieServers(Vec<Ipv4Addr>), // code 8
  LprServers(Vec<Ipv4Addr>), // code 9
  ImpressServers(Vec<Ipv4Addr>), // code 10
  ResourceLocationServers(Vec<Ipv4Addr>), // code 11

  HostName(String), // code 12 // The name of the client may or may not be qualified with the local domain name (it is recommended that the client send the fully-qualified domain name). The code for this option is 12, and its minimum length is 1
  BootFileSize(u16), // code 13
  MeritDumpFile(String), // code 14 // This option specifies the path-name of a file to which the client's core image should be dumped in the event the client crashes.
  DomainName(String), // code 15 // This option specifies the client's domain name.
  SwapServer(Ipv4Addr), // code 16
  RootPath(String), // code 17
  ExtensionsPath(String), // code 18

  // IP Layer Parameters per Host
  // code 19 to 25
  // IP Layer Parameters per Interface
  // code 26 to 33
  // Link Layer Parameters per Interface
  // code 34 to 36
  // TCP Parameters
  // code 37 to 39
  // Application and Service Parameters
  // code 40 to 42
  // Vendor Specific Information (Add in RFC 2132)
  // code 43-49 and 64-65 68-76
  // TODO Unrecognized options will be recognized as Unrecognized(RawDhcpOption)
  // including some important options e.g. MTU/ARP/NTR/Static Route
  Unrecognized,

  // DHCP Extensions
  // code 50 to 61
  RequestedIpAddress(Ipv4Addr), // code 50 // This option is used in a client request (DHCPDISCOVER) to allow the client to request that a particular IP address be assigned.
  IpAddressLeaseTime(u32), // code 51 // This option is used in a client request (DHCPDISCOVER or DHCPREQUEST) to allow the client to request a lease time for the IP address.  In a server reply (DHCPOFFER), a DHCP server uses this option to specify the lease time it is willing to offer.
  OptionOverload(OptionOverLoadCode), // code 52 // The code for this option is 52, and its length is 1.  Legal values for this option are: 1 2 3. This option is used to indicated that the DHCP "sname" or "file" fields are being overloaded by using them to carry DHCP options.
  DhcpMessageType(DhcpMessageTypeCode), // code 53
  ServerIdentifier(Ipv4Addr), // code 54
  ParameterRequestList(Vec<u8>),   // code 55
  // or Message(String)
  // Message(Vec<u8>), // code 56
  Message(String), // code 56
  MaximumDhcpMessageSize(u16), // code 57  minimum 576
  RenewalTimeValue(u32), // code 58 32-bit unsigned integer
  RebindingTimeValue(u32), // code 59 32-bit unsigned integer
  ClassIdentifier(Vec<u8>), // code 60 minimum 1 octet class identifier. Servers not equipped to interpret the class-specific information sent by a client MUST ignore it (although it may be reported). // May carry a customized messages with username and p\*w$#d for authentication like IPoE protocl

  ClientIdentifier(Vec<u8>), // code 61 minimum 2 octets. The client identifier is used by the client to pass its unique identifier to the server. See HostName option.


}
pub enum OptionOverLoadCode {
  OverloadFile = 1,
  OverloadSname = 2,
  OverloadBoth = 3,
}
pub enum DhcpMessageTypeCode {
    Discover = 1,
    Offer = 2,
    Request = 3,
    Decline = 4,
    Ack = 5,
    Nak = 6,
    Release = 7,
}

// DHCP Options;
pub const PAD_OPTION: u8 = 0;
pub const END_OPTION: u8 = 255;
pub const SUBNET_MASK: u8 = 1;
pub const TIME_OFFSET: u8 = 2;
pub const ROUTERS: u8 = 3;
pub const TIME_SERVERS: u8 = 4;
pub const NAME_SERVERS: u8 = 5;
pub const DOMAIN_NAME_SERVERS: u8 = 6;
pub const LOG_SERVERS: u8 = 7;
pub const COOKIE_SERVERS: u8 = 8;
pub const LPR_SERVERS: u8 = 9;
pub const IMPRESS_SERVERS: u8 = 10;
pub const RESOURCE_LOCATION_SERVERS: u8 = 11;
pub const HOST_NAME: u8 = 12;
pub const BOOT_FILE_SIZE: u8 = 13;
pub const MERIT_DUMP_FILE: u8 = 14;
pub const DOMAIN_NAME: u8 = 15;
pub const SWAP_SERVER: u8 = 16;
pub const ROOT_PATH: u8 = 17;
pub const EXTENSIONS_PATH: u8 = 18;

// IP LAYER PARAMETERS PER HOST;
pub const IP_FORWARDING: u8 = 19;
pub const NON_LOCAL_SOURCE_ROUTING: u8 = 20;
pub const POLICY_FILTER: u8 = 21;
pub const MAXIMUM_DATAGRAM_REASSEMBLY_SIZE: u8 = 22;
pub const DEFAULT_IP_TTL: u8 = 23;
pub const PATH_MTU_AGING_TIMEOUT: u8 = 24;
pub const PATH_MTU_PLATEAU_TABLE: u8 = 25;

// IP LAYER PARAMETERS PER INTERFACE;
pub const INTERFACE_MTU: u8 = 26;
pub const ALL_SUBNETS_ARE_LOCAL: u8 = 27;
pub const BROADCAST_ADDRESS: u8 = 28;
pub const PERFORM_MASK_DISCOVERY: u8 = 29;
pub const MASK_SUPPLIER: u8 = 30;
pub const PERFORM_ROUTER_DISCOVERY: u8 = 31;
pub const ROUTER_SOLICITATION_ADDRESS: u8 = 32;
pub const STATIC_ROUTE: u8 = 33;

// LINK LAYER PARAMETERS PER INTERFACE;
pub const TRAILER_ENCAPSULATION: u8 = 34;
pub const ARP_CACHE_TIMEOUT: u8 = 35;
pub const ETHERNET_ENCAPSULATION: u8 = 36;

// TCP PARAMETERS;
pub const TCP_DEFAULT_TTL: u8 = 37;
pub const TCP_KEEPALIVE_INTERVAL: u8 = 38;
pub const TCP_KEEPALIVE_GARBAGE: u8 = 39;

// APPLICATION AND SERVICE PARAMETERS;
pub const NETWORK_INFORMATION_SERVICE_DOMAIN: u8 = 40;
pub const NETWORK_INFORMATION_SERVERS: u8 = 41;
pub const NETWORK_TIME_PROTOCOL_SERVERS: u8 = 42;

// VENDOR SPECIFIC INFORMATION;
pub const VENDOR_SPECIFIC_INFORMATION: u8 = 43;
pub const NETBIOS_OVER_TCPIP_NAME_SERVER: u8 = 44;
pub const NETBIOS_OVER_TCPIP_DATAGRAM_DISTRIBUTION_SERVER: u8 = 45;
pub const NETBIOS_OVER_TCPIP_NODE_TYPE: u8 = 46;
pub const NETBIOS_OVER_TCPIP_SCOPE: u8 = 47;
pub const XWINDOW_SYSTEM_FONT_SERVER: u8 = 48;
pub const XWINDOW_SYSTEM_DISPLAY_MANAGER: u8 = 49;
pub const NETWORK_INFORMATION_SERVICEPLUS_DOMAIN: u8 = 64;
pub const NETWORK_INFORMATION_SERVICEPLUS_SERVERS: u8 = 65;
pub const MOBILE_IP_HOME_AGENT: u8 = 68;
pub const SIMPLE_MAIL_TRANSPORT_PROTOCOL: u8 = 69;
pub const POST_OFFICE_PROTOCOL_SERVER: u8 = 70;
pub const NETWORK_NEWS_TRANSPORT_PROTOCOL: u8 = 71;
pub const DEFAULT_WORLD_WIDE_WEB_SERVER: u8 = 72;
pub const DEFAULT_FINGER_SERVER: u8 = 73;
pub const DEFAULT_INTERNET_RELAY_CHAT_SERVER: u8 = 74;
pub const STREETTALK_SERVER: u8 = 75;
pub const STREETTALK_DIRECTORY_ASSISTANCE: u8 = 76;

pub const RELAY_AGENT_INFORMATION: u8 = 82;

// DHCP EXTENSIONS
pub const REQUESTED_IP_ADDRESS: u8 = 50;
pub const IP_ADDRESS_LEASE_TIME: u8 = 51;
pub const OPTION_OVERLOAD: u8 = 52;
pub const DHCP_MESSAGE_TYPE: u8 = 53;
pub const SERVER_IDENTIFIER: u8 = 54;
pub const PARAMETER_REQUEST_LIST: u8 = 55;
pub const MESSAGE: u8 = 56;
pub const MAXIMUM_DHCP_MESSAGE_SIZE: u8 = 57;
pub const RENEWAL_TIME_VALUE: u8 = 58;
pub const REBINDING_TIME_VALUE: u8 = 59;
pub const CLASS_IDENTIFIER: u8 = 60;
pub const CLIENT_IDENTIFIER: u8 = 61;

pub const TFTP_SERVER_NAME: u8 = 66;
pub const BOOTFILE_NAME: u8 = 67;

pub const USER_CLASS: u8 = 77;
// No support for DHCPv4 options in [RFC 4702](https://datatracker.ietf.org/doc/html/rfc4702)
// pub const CLIENT_FQDN: u8 = 81;
pub const CLIENT_ARCHITECTURE: u8 = 93;
pub const TZ_POSIX_STRING: u8 = 100;
pub const TZ_DATABASE_STRING: u8 = 101;

// Add in RFC 3442 (obsoletes the static route option in RFC 2132 option 33)
pub const CLASSLESS_ROUTE_FORMAT: u8 = 121;

// No support for DHCPv4 options in [RFC 3925](https://datatracker.ietf.org/doc/html/rfc3925)
// E.g. option code 120 (SIP) 129 143 184

/// Returns title of DHCP Option code, if known.
pub fn title(code: u8) -> Option<&'static str> {
    Some(match code {
        SUBNET_MASK => "Subnet Mask",

        TIME_OFFSET => "Time Offset",
        ROUTERS => "Router",
        TIME_SERVERS => "Time Server",
        NAME_SERVERS => "Name Server",
        DOMAIN_NAME_SERVERS => "Domain Name Server",
        LOG_SERVERS => "Log Server",
        COOKIE_SERVERS => "Cookie Server",
        LPR_SERVERS => "LPR Server",
        IMPRESS_SERVERS => "Impress Server",
        RESOURCE_LOCATION_SERVERS => "Resource Location Server",
        HOST_NAME => "Host Name",
        BOOT_FILE_SIZE => "Boot File Size",
        MERIT_DUMP_FILE => "Merit Dump File",
        DOMAIN_NAME => "Domain Name",
        SWAP_SERVER => "Swap Server",
        ROOT_PATH => "Root Path",
        EXTENSIONS_PATH => "Extensions Path",

        // IP LAYER PARAMETERS PER HOST",
        IP_FORWARDING => "IP Forwarding Enable/Disable",
        NON_LOCAL_SOURCE_ROUTING => "Non-Local Source Routing Enable/Disable",
        POLICY_FILTER => "Policy Filter",
        MAXIMUM_DATAGRAM_REASSEMBLY_SIZE => "Maximum Datagram Reassembly Size",
        DEFAULT_IP_TTL => "Default IP Time-to-live",
        PATH_MTU_AGING_TIMEOUT => "Path MTU Aging Timeout",
        PATH_MTU_PLATEAU_TABLE => "Path MTU Plateau Table",

        // IP LAYER PARAMETERS PER INTERFACE",
        INTERFACE_MTU => "Interface MTU",
        ALL_SUBNETS_ARE_LOCAL => "All Subnets are Local",
        BROADCAST_ADDRESS => "Broadcast Address",
        PERFORM_MASK_DISCOVERY => "Perform Mask Discovery",
        MASK_SUPPLIER => "Mask Supplier",
        PERFORM_ROUTER_DISCOVERY => "Perform Router Discovery",
        ROUTER_SOLICITATION_ADDRESS => "Router Solicitation Address",
        STATIC_ROUTE => "Static Route",

        // LINK LAYER PARAMETERS PER INTERFACE",
        TRAILER_ENCAPSULATION => "Trailer Encapsulation",
        ARP_CACHE_TIMEOUT => "ARP Cache Timeout",
        ETHERNET_ENCAPSULATION => "Ethernet Encapsulation",

        // TCP PARAMETERS",
        TCP_DEFAULT_TTL => "TCP Default TTL",
        TCP_KEEPALIVE_INTERVAL => "TCP Keepalive Interval",
        TCP_KEEPALIVE_GARBAGE => "TCP Keepalive Garbage",

        // APPLICATION AND SERVICE PARAMETERS",
        NETWORK_INFORMATION_SERVICE_DOMAIN => "Network Information Service Domain",
        NETWORK_INFORMATION_SERVERS => "Network Information Servers",
        NETWORK_TIME_PROTOCOL_SERVERS => "Network Time Protocol Servers",
        VENDOR_SPECIFIC_INFORMATION => "Vendor Specific Information",
        NETBIOS_OVER_TCPIP_NAME_SERVER => "NetBIOS over TCP/IP Name Server",
        NETBIOS_OVER_TCPIP_DATAGRAM_DISTRIBUTION_SERVER => {
            "NetBIOS over TCP/IP Datagram Distribution Server"
        }
        NETBIOS_OVER_TCPIP_NODE_TYPE => "NetBIOS over TCP/IP Node Type",
        NETBIOS_OVER_TCPIP_SCOPE => "NetBIOS over TCP/IP Scope",
        XWINDOW_SYSTEM_FONT_SERVER => "X Window System Font Server",
        XWINDOW_SYSTEM_DISPLAY_MANAGER => "X Window System Display Manager",
        NETWORK_INFORMATION_SERVICEPLUS_DOMAIN => "Network Information Service+ Domain",
        NETWORK_INFORMATION_SERVICEPLUS_SERVERS => "Network Information Service+ Servers",
        MOBILE_IP_HOME_AGENT => "Mobile IP Home Agent",
        SIMPLE_MAIL_TRANSPORT_PROTOCOL => "Simple Mail Transport Protocol (SMTP) Server",
        POST_OFFICE_PROTOCOL_SERVER => "Post Office Protocol (POP3) Server",
        NETWORK_NEWS_TRANSPORT_PROTOCOL => "Network News Transport Protocol (NNTP) Server",
        DEFAULT_WORLD_WIDE_WEB_SERVER => "Default World Wide Web (WWW) Server",
        DEFAULT_FINGER_SERVER => "Default Finger Server",
        DEFAULT_INTERNET_RELAY_CHAT_SERVER => "Default Internet Relay Chat (IRC) Server",
        STREETTALK_SERVER => "StreetTalk Server",
        STREETTALK_DIRECTORY_ASSISTANCE => "StreetTalk Directory Assistance (STDA) Server",

        RELAY_AGENT_INFORMATION => "Relay Agent Information",

        // DHCP EXTENSIONS
        REQUESTED_IP_ADDRESS => "Requested IP Address",
        IP_ADDRESS_LEASE_TIME => "IP Address Lease Time",
        OPTION_OVERLOAD => "Option Overload",
        DHCP_MESSAGE_TYPE => "DHCP Message Type",
        SERVER_IDENTIFIER => "Server Identifier",
        PARAMETER_REQUEST_LIST => "Parameter Request List",
        MESSAGE => "Message",
        MAXIMUM_DHCP_MESSAGE_SIZE => "Maximum DHCP Message Size",
        RENEWAL_TIME_VALUE => "Renewal (T1) Time Value",
        REBINDING_TIME_VALUE => "Rebinding (T2) Time Value",
        CLASS_IDENTIFIER => "Class-identifier (Vendor class-identifier)",
        CLIENT_IDENTIFIER => "Client-identifier",

        // Find below
        TFTP_SERVER_NAME => "TFTP server name",
        BOOTFILE_NAME => "Bootfile name",

        USER_CLASS => "User Class",

        CLIENT_ARCHITECTURE => "Client Architecture",

        TZ_POSIX_STRING => "TZ-POSIX String",
        TZ_DATABASE_STRING => "TZ-Database String",
        CLASSLESS_ROUTE_FORMAT => "Classless Route Format",

        _ => return None,
    })
}


pub fn test_options() {
    println!("[TEST] test_options");
}