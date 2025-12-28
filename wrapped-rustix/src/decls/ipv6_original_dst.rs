macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ipv6_original_dst {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IPV6, IP6T_SO_ORIGINAL_DST)`"] # [doc = ""] # [doc = " Even though this corresponds to a `IP6T_*` constant, it is an"] # [doc = " `IPPROTO_IPV6` option."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "IP6T_SO_ORIGINAL_DST")] pub fn ipv6_original_dst < Fd : AsFd > (fd : Fd) -> io :: Result < SocketAddrV6 > { backend :: net :: sockopt :: ipv6_original_dst (fd . as_fd ()) }
    };
}

ipv6_original_dst!()