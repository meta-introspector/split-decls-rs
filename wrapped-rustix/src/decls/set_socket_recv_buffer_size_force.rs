macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_socket_recv_buffer_size_force {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, SO_RCVBUFFORCE, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (any (linux_kernel , target_os = "fuchsia" , target_os = "redox"))] # [inline] # [doc (alias = "SO_RCVBUFFORCE")] pub fn set_socket_recv_buffer_size_force < Fd : AsFd > (fd : Fd , value : usize) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_recv_buffer_size_force (fd . as_fd () , value) }
    };
}

set_socket_recv_buffer_size_force!();