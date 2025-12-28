macro_rules! deps {
    () => {
        Result!();
        Timeout!();
    };
}

macro_rules! set_socket_timeout {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, id, value)`—Set the sending or receiving"] # [doc = " timeout."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_RCVTIMEO")] # [doc (alias = "SO_SNDTIMEO")] pub fn set_socket_timeout < Fd : AsFd > (fd : Fd , id : Timeout , value : Option < Duration > ,) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_timeout (fd . as_fd () , id , value) }
    };
}

set_socket_timeout!();