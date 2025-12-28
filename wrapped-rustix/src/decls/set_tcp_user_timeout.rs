macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_tcp_user_timeout {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_USER_TIMEOUT, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_USER_TIMEOUT")] pub fn set_tcp_user_timeout < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_user_timeout (fd . as_fd () , value) }
    };
}

set_tcp_user_timeout!()