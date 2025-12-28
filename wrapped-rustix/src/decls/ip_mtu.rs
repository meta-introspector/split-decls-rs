macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ip_mtu {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IP, IP_MTU)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [cfg (any (linux_kernel , target_os = "cygwin"))] # [doc (alias = "IP_MTU")] pub fn ip_mtu < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: ip_mtu (fd . as_fd ()) }
    };
}

ip_mtu!();