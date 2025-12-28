macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_socket_nosigpipe {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, SO_NOSIGPIPE, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (any (apple , freebsdlike , target_os = "netbsd"))] # [doc (alias = "SO_NOSIGPIPE")] # [inline] pub fn set_socket_nosigpipe < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_nosigpipe (fd . as_fd () , value) }
    };
}

set_socket_nosigpipe!();