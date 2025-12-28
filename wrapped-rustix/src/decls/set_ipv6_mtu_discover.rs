macro_rules! deps {
    () => {
        Ipv6PathMtuDiscovery!();
        Result!();
    };
}

macro_rules! set_ipv6_mtu_discover {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IPV6, IPV6_MTU_DISCOVER, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "IPV6_MTU_DISCOVER")] pub fn set_ipv6_mtu_discover < Fd : AsFd > (fd : Fd , value : Ipv6PathMtuDiscovery) -> io :: Result < () > { backend :: net :: sockopt :: set_ipv6_mtu_discover (fd . as_fd () , value) }
    };
}

set_ipv6_mtu_discover!()