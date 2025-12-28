macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_socket_recv_buffer_size {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, SO_RCVBUF, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_RCVBUF")] pub fn set_socket_recv_buffer_size < Fd : AsFd > (fd : Fd , value : usize) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_recv_buffer_size (fd . as_fd () , value) }
    };
}

set_socket_recv_buffer_size!();