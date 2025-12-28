macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_ipv6_multicast_loop {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_LOOP, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [inline] # [doc (alias = "IPV6_MULTICAST_LOOP")] pub fn set_ipv6_multicast_loop < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_ipv6_multicast_loop (fd . as_fd () , value) }
    };
}

set_ipv6_multicast_loop!()