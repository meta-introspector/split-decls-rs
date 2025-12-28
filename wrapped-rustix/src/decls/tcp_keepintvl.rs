macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! tcp_keepintvl {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_KEEPINTVL)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (not (any (target_os = "haiku" , target_os = "nto" , target_os = "openbsd" , target_os = "redox")))] # [inline] # [doc (alias = "TCP_KEEPINTVL")] pub fn tcp_keepintvl < Fd : AsFd > (fd : Fd) -> io :: Result < Duration > { backend :: net :: sockopt :: tcp_keepintvl (fd . as_fd ()) }
    };
}

tcp_keepintvl!();