// Generated macro for parse_interface_address (function)
macro_rules! Depcrate_unix_network_helperparse_interface_address {
() => {
// Module: crate::unix::network_helper
// Provides: {"parse_interface_address"}
// Dependencies: {}
# [cfg (any (target_os = "linux" , target_os = "android"))] unsafe fn parse_interface_address (ifap : & libc :: ifaddrs) -> Option < MacAddr > { use libc :: sockaddr_ll ; let sock_addr = ifap . ifa_addr ; if sock_addr . is_null () { return None ; } unsafe { match (* sock_addr) . sa_family as libc :: c_int { libc :: AF_PACKET => { let addr = sock_addr as * const sockaddr_ll ; let [addr @ .. , _ , _] = (* addr) . sll_addr ; Some (MacAddr (addr)) } _ => None , } } }
};
}
