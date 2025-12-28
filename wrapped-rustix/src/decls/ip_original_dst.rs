macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ip_original_dst {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IP, SO_ORIGINAL_DST)`"] # [doc = ""] # [doc = " Even though this corresponds to a `SO_*` constant, it is an `IPPROTO_IP`"] # [doc = " option."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (any (linux_kernel , target_os = "fuchsia"))] # [inline] # [doc (alias = "SO_ORIGINAL_DST")] pub fn ip_original_dst < Fd : AsFd > (fd : Fd) -> io :: Result < SocketAddrV4 > { backend :: net :: sockopt :: ip_original_dst (fd . as_fd ()) }
    };
}

ip_original_dst!();