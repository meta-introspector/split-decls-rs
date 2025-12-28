macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! socket_recv_buffer_size {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_RCVBUF)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_RCVBUF")] pub fn socket_recv_buffer_size < Fd : AsFd > (fd : Fd) -> io :: Result < usize > { backend :: net :: sockopt :: socket_recv_buffer_size (fd . as_fd ()) }
    };
}

socket_recv_buffer_size!()