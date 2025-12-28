macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_socket_oobinline {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, SO_OOBINLINE, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_OOBINLINE")] pub fn set_socket_oobinline < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_oobinline (fd . as_fd () , value) }
    };
}

set_socket_oobinline!();