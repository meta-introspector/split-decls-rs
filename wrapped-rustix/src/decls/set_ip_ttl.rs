macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_ip_ttl {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IP, IP_TTL, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "IP_TTL")] pub fn set_ip_ttl < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_ttl (fd . as_fd () , value) }
    };
}

set_ip_ttl!()