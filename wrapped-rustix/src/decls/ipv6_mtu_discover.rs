macro_rules! deps {
    () => {
        Result!();
        Ipv6PathMtuDiscovery!();
    };
}

macro_rules! ipv6_mtu_discover {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_MTU_DISCOVER)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "IPV6_MTU_DISCOVER")] pub fn ipv6_mtu_discover < Fd : AsFd > (fd : Fd) -> io :: Result < Ipv6PathMtuDiscovery > { backend :: net :: sockopt :: ipv6_mtu_discover (fd . as_fd ()) }
    };
}

ipv6_mtu_discover!()