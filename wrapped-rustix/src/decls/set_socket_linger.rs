macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_socket_linger {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, SO_LINGER, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_LINGER")] pub fn set_socket_linger < Fd : AsFd > (fd : Fd , value : Option < Duration >) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_linger (fd . as_fd () , value) }
    };
}

set_socket_linger!()