macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! socket_keepalive {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_KEEPALIVE)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_KEEPALIVE")] pub fn socket_keepalive < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_keepalive (fd . as_fd ()) }
    };
}

socket_keepalive!();