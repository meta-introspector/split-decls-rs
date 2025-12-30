// Generated macro for get_interface_ip_networks (function)
macro_rules! Depcrate_unix_network_helperget_interface_ip_networks {
() => {
// Module: crate::unix::network_helper
// Provides: {"get_interface_ip_networks"}
// Dependencies: {}
pub (crate) unsafe fn get_interface_ip_networks () -> HashMap < String , HashSet < IpNetwork > > { let mut ifaces : HashMap < String , HashSet < IpNetwork > > = HashMap :: new () ; let mut addrs : MaybeUninit < * mut libc :: ifaddrs > = MaybeUninit :: uninit () ; if unsafe { libc :: getifaddrs (addrs . as_mut_ptr ()) } != 0 { sysinfo_debug ! ("Failed to operate libc::getifaddrs as ifaddrs Uninitialized") ; return ifaces ; } let addrs = unsafe { addrs . assume_init () } ; let mut addr = addrs ; while ! addr . is_null () { let addr_ref : & libc :: ifaddrs = unsafe { & * addr } ; let c_str = addr_ref . ifa_name as * const c_char ; let bytes = unsafe { CStr :: from_ptr (c_str) . to_bytes () } ; let mut name = unsafe { from_utf8_unchecked (bytes) . to_owned () } ; if name . contains (':') { if let Some (interface_name) = name . split (':') . next () { name = interface_name . to_string () } else { addr = addr_ref . ifa_next ; continue ; } } let ip = sockaddr_to_network_addr (addr_ref . ifa_addr as * const libc :: sockaddr) ; let netmask = sockaddr_to_network_addr (addr_ref . ifa_netmask as * const libc :: sockaddr) ; let prefix = netmask . and_then (| netmask | ip_mask_to_prefix (netmask) . ok ()) . unwrap_or (0) ; if let Some (ip) = ip { ifaces . entry (name) . and_modify (| values | { values . insert (IpNetwork { addr : ip , prefix }) ; }) . or_insert (HashSet :: from ([IpNetwork { addr : ip , prefix }])) ; } addr = addr_ref . ifa_next ; } unsafe { libc :: freeifaddrs (addrs) } ; ifaces }
};
}
