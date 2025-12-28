macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! socket_reuseaddr {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_REUSEADDR)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_REUSEADDR")] pub fn socket_reuseaddr < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_reuseaddr (fd . as_fd ()) }
    };
}

socket_reuseaddr!()