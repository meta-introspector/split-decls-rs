macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! tcp_keepcnt {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_KEEPCNT)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (not (any (target_os = "haiku" , target_os = "nto" , target_os = "openbsd" , target_os = "redox")))] # [inline] # [doc (alias = "TCP_KEEPCNT")] pub fn tcp_keepcnt < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: tcp_keepcnt (fd . as_fd ()) }
    };
}

tcp_keepcnt!()