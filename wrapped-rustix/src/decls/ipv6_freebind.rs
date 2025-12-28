macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ipv6_freebind {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_FREEBIND)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "IPV6_FREEBIND")] pub fn ipv6_freebind < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: ipv6_freebind (fd . as_fd ()) }
    };
}

ipv6_freebind!();