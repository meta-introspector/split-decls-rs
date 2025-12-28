macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_tcp_nodelay {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [inline] # [doc (alias = "TCP_NODELAY")] pub fn set_tcp_nodelay < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_nodelay (fd . as_fd () , value) }
    };
}

set_tcp_nodelay!();