// Generated macro for Ipv4PathMtuDiscovery (struct)
macro_rules! Depcrate_net_sockoptIpv4PathMtuDiscovery {
() => {
// Module: crate::net::sockopt
// Provides: {"Ipv4PathMtuDiscovery"}
// Dependencies: {}
# [doc = " IPv4 Path MTU Discovery option values (`IP_PMTUDISC_*`) for use with"] # [doc = " [`set_ip_mtu_discover`] and [`ip_mtu_discover`]."] # [doc = ""] # [doc = " # References"] # [doc = " - [Linux]"] # [doc = " - [Linux INET header]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man7/ip.7.html"] # [doc = " [Linux INET header]: https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/linux/in.h?h=v6.14#n135"] # [cfg (linux_kernel)] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (transparent)] pub struct Ipv4PathMtuDiscovery (RawIpv4PathMtuDiscovery) ;
};
}
