macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! tcp_keepidle {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_KEEPIDLE)`"] # [doc = ""] # [doc = " `TCP_KEEPALIVE` on Apple platforms."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (not (any (target_os = "haiku" , target_os = "nto" , target_os = "openbsd")))] # [inline] # [doc (alias = "TCP_KEEPIDLE")] pub fn tcp_keepidle < Fd : AsFd > (fd : Fd) -> io :: Result < Duration > { backend :: net :: sockopt :: tcp_keepidle (fd . as_fd ()) }
    };
}

tcp_keepidle!()