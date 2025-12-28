macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! socket_broadcast {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_BROADCAST)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_BROADCAST")] pub fn socket_broadcast < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_broadcast (fd . as_fd ()) }
    };
}

socket_broadcast!();