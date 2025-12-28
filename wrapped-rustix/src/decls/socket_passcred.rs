macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! socket_passcred {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_PASSCRED)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "SO_PASSCRED")] pub fn socket_passcred < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_passcred (fd . as_fd ()) }
    };
}

socket_passcred!()