macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! socket_incoming_cpu {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_INCOMING_CPU)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (target_os = "linux")] # [inline] # [doc (alias = "SO_INCOMING_CPU")] pub fn socket_incoming_cpu < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: socket_incoming_cpu (fd . as_fd ()) }
    };
}

socket_incoming_cpu!();