macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ip_freebind {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IP, IP_FREEBIND)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (any (linux_kernel , target_os = "fuchsia"))] # [inline] # [doc (alias = "IP_FREEBIND")] pub fn ip_freebind < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: ip_freebind (fd . as_fd ()) }
    };
}

ip_freebind!()