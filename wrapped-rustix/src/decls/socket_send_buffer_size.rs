macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! socket_send_buffer_size {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_SNDBUF)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_SNDBUF")] pub fn socket_send_buffer_size < Fd : AsFd > (fd : Fd) -> io :: Result < usize > { backend :: net :: sockopt :: socket_send_buffer_size (fd . as_fd ()) }
    };
}

socket_send_buffer_size!()