macro_rules! deps {
    () => {
        Timeout!();
        Result!();
    };
}

macro_rules! socket_timeout {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, id)`—Get the sending or receiving timeout."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_RCVTIMEO")] # [doc (alias = "SO_SNDTIMEO")] pub fn socket_timeout < Fd : AsFd > (fd : Fd , id : Timeout) -> io :: Result < Option < Duration > > { backend :: net :: sockopt :: socket_timeout (fd . as_fd () , id) }
    };
}

socket_timeout!()