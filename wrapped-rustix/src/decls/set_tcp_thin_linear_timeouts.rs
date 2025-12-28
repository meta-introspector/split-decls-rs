macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_tcp_thin_linear_timeouts {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_THIN_LINEAR_TIMEOUTS, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_THIN_LINEAR_TIMEOUTS")] pub fn set_tcp_thin_linear_timeouts < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_thin_linear_timeouts (fd . as_fd () , value) }
    };
}

set_tcp_thin_linear_timeouts!()