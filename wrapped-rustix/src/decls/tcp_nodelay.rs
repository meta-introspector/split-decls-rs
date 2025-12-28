macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! tcp_nodelay {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_NODELAY)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [inline] # [doc (alias = "TCP_NODELAY")] pub fn tcp_nodelay < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: tcp_nodelay (fd . as_fd ()) }
    };
}

tcp_nodelay!();