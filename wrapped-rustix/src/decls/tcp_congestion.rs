macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! tcp_congestion {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_CONGESTION)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (feature = "alloc")] # [cfg (any (linux_like , target_os = "freebsd" , target_os = "fuchsia" , target_os = "illumos"))] # [inline] # [doc (alias = "TCP_CONGESTION")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn tcp_congestion < Fd : AsFd > (fd : Fd) -> io :: Result < String > { backend :: net :: sockopt :: tcp_congestion (fd . as_fd ()) }
    };
}

tcp_congestion!();