macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_tcp_keepintvl {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_KEEPINTVL, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (not (any (target_os = "haiku" , target_os = "nto" , target_os = "openbsd" , target_os = "redox")))] # [inline] # [doc (alias = "TCP_KEEPINTVL")] pub fn set_tcp_keepintvl < Fd : AsFd > (fd : Fd , value : Duration) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_keepintvl (fd . as_fd () , value) }
    };
}

set_tcp_keepintvl!()