macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_ipv6_add_membership {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IPV6, IPV6_ADD_MEMBERSHIP, multiaddr, interface)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [inline] # [doc (alias = "IPV6_JOIN_GROUP")] # [doc (alias = "IPV6_ADD_MEMBERSHIP")] pub fn set_ipv6_add_membership < Fd : AsFd > (fd : Fd , multiaddr : & Ipv6Addr , interface : u32 ,) -> io :: Result < () > { backend :: net :: sockopt :: set_ipv6_add_membership (fd . as_fd () , multiaddr , interface) }
    };
}

set_ipv6_add_membership!()