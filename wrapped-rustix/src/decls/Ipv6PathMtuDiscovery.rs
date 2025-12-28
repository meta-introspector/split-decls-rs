macro_rules! deps {
    () => {
        RawIpv6PathMtuDiscovery!();
    };
}

macro_rules! Ipv6PathMtuDiscovery {
    () => {
        deps!();
        # [doc = " IPv6 Path MTU Discovery option values (`IPV6_PMTUDISC_*`) for use with"] # [doc = " [`set_ipv6_mtu_discover`] and [`ipv6_mtu_discover`]."] # [doc = ""] # [doc = " # References"] # [doc = " - [Linux]"] # [doc = " - [Linux INET6 header]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man7/ipv6.7.html"] # [doc = " [Linux INET6 header]: https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/linux/in6.h?h=v6.14#n185"] # [cfg (linux_kernel)] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (transparent)] pub struct Ipv6PathMtuDiscovery (RawIpv6PathMtuDiscovery) ;
    };
}

Ipv6PathMtuDiscovery!()