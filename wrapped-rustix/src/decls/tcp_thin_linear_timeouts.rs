macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! tcp_thin_linear_timeouts {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_THIN_LINEAR_TIMEOUTS)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_THIN_LINEAR_TIMEOUTS")] pub fn tcp_thin_linear_timeouts < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: tcp_thin_linear_timeouts (fd . as_fd ()) }
    };
}

tcp_thin_linear_timeouts!()