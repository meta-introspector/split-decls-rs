macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! tcp_cork {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_CORK)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , solarish , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_CORK")] pub fn tcp_cork < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: tcp_cork (fd . as_fd ()) }
    };
}

tcp_cork!();