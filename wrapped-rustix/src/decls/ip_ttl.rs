macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ip_ttl {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IP, IP_TTL)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_TTL")] pub fn ip_ttl < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: ip_ttl (fd . as_fd ()) }
    };
}

ip_ttl!()