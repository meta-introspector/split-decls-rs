macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_ip_freebind {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IP, IP_FREEBIND, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (any (linux_kernel , target_os = "fuchsia"))] # [inline] # [doc (alias = "IP_FREEBIND")] pub fn set_ip_freebind < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_freebind (fd . as_fd () , value) }
    };
}

set_ip_freebind!()