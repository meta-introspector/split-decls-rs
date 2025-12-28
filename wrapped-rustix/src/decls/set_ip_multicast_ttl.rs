macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_ip_multicast_ttl {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IP, IP_MULTICAST_TTL, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_MULTICAST_TTL")] pub fn set_ip_multicast_ttl < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_multicast_ttl (fd . as_fd () , value) }
    };
}

set_ip_multicast_ttl!();