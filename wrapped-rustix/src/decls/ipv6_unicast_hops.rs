macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ipv6_unicast_hops {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_UNICAST_HOPS)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [inline] # [doc (alias = "IPV6_UNICAST_HOPS")] pub fn ipv6_unicast_hops < Fd : AsFd > (fd : Fd) -> io :: Result < u8 > { backend :: net :: sockopt :: ipv6_unicast_hops (fd . as_fd ()) }
    };
}

ipv6_unicast_hops!();