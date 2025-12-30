// Generated macro for get_ip_address_from_socket_address (function)
macro_rules! Depcrate_windows_network_helperget_ip_address_from_socket_address {
() => {
// Module: crate::windows::network_helper
// Provides: {"get_ip_address_from_socket_address"}
// Dependencies: {}
# [doc = " Converts a Windows socket address to an ip address."] fn get_ip_address_from_socket_address (socket_address : NonNull < SOCKADDR >) -> Option < IpAddr > { let socket_address_family = unsafe { socket_address . as_ref () . sa_family } ; match socket_address_family { AF_INET => { let socket_address = unsafe { socket_address . cast :: < SOCKADDR_IN > () . as_ref () } ; let address = unsafe { socket_address . sin_addr . S_un . S_addr } ; let ipv4_address = IpAddr :: from (address . to_ne_bytes ()) ; Some (ipv4_address) } AF_INET6 => { let socket_address = unsafe { socket_address . cast :: < SOCKADDR_IN6 > () . as_ref () } ; let address = unsafe { socket_address . sin6_addr . u . Byte } ; let ipv6_address = IpAddr :: from (address) ; Some (ipv6_address) } _ => None , } }
};
}
