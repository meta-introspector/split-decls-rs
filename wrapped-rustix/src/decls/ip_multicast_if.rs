macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ip_multicast_if {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IP, IP_MULTICAST_IF)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_MULTICAST_IF")] pub fn ip_multicast_if < Fd : AsFd > (fd : Fd) -> io :: Result < Ipv4Addr > { backend :: net :: sockopt :: ip_multicast_if (fd . as_fd ()) }
    };
}

ip_multicast_if!();