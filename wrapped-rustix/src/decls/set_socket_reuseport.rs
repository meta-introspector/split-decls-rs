macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_socket_reuseport {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, SO_REUSEPORT, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (not (any (solarish , windows , target_os = "cygwin")))] # [cfg (not (windows))] # [inline] # [doc (alias = "SO_REUSEPORT")] pub fn set_socket_reuseport < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_reuseport (fd . as_fd () , value) }
    };
}

set_socket_reuseport!()