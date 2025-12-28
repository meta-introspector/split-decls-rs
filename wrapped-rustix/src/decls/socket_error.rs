macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! socket_error {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_ERROR)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_ERROR")] pub fn socket_error < Fd : AsFd > (fd : Fd) -> io :: Result < Result < () , io :: Errno > > { backend :: net :: sockopt :: socket_error (fd . as_fd ()) }
    };
}

socket_error!();