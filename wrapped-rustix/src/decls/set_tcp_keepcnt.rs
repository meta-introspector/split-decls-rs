macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_tcp_keepcnt {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_KEEPCNT, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (not (any (target_os = "haiku" , target_os = "nto" , target_os = "openbsd" , target_os = "redox")))] # [inline] # [doc (alias = "TCP_KEEPCNT")] pub fn set_tcp_keepcnt < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_keepcnt (fd . as_fd () , value) }
    };
}

set_tcp_keepcnt!()