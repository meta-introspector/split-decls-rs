macro_rules! deps {
    () => {
        Result!();
        SocketType!();
    };
}

macro_rules! socket_type {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_TYPE)`—Returns the type of a socket."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_TYPE")] pub fn socket_type < Fd : AsFd > (fd : Fd) -> io :: Result < SocketType > { backend :: net :: sockopt :: socket_type (fd . as_fd ()) }
    };
}

socket_type!()