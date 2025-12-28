macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_socket_broadcast {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, SO_BROADCAST, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_BROADCAST")] pub fn set_socket_broadcast < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_broadcast (fd . as_fd () , value) }
    };
}

set_socket_broadcast!();