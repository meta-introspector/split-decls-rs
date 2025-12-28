macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ipv6_multicast_hops {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_HOPS)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [inline] # [doc (alias = "IPV6_MULTICAST_HOPS")] pub fn ipv6_multicast_hops < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: ipv6_multicast_hops (fd . as_fd ()) }
    };
}

ipv6_multicast_hops!()