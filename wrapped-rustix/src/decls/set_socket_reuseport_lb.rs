macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_socket_reuseport_lb {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, SO_REUSEPORT_LB, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (target_os = "freebsd")] # [inline] # [doc (alias = "SO_REUSEPORT_LB")] pub fn set_socket_reuseport_lb < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_reuseport_lb (fd . as_fd () , value) }
    };
}

set_socket_reuseport_lb!()