macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_tcp_congestion {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_CONGESTION, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "freebsd" , target_os = "fuchsia" , target_os = "illumos"))] # [inline] # [doc (alias = "TCP_CONGESTION")] pub fn set_tcp_congestion < Fd : AsFd > (fd : Fd , value : & str) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_congestion (fd . as_fd () , value) }
    };
}

set_tcp_congestion!()