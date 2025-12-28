macro_rules! deps {
    () => {
        UCred!();
        Result!();
    };
}

macro_rules! socket_peercred {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_PEERCRED)`—Get credentials of Unix domain"] # [doc = " socket peer process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux `unix`]"] # [doc = ""] # [doc = " [Linux `unix`]: https://man7.org/linux/man-pages/man7/unix.7.html"] # [cfg (linux_kernel)] # [doc (alias = "SO_PEERCRED")] pub fn socket_peercred < Fd : AsFd > (fd : Fd) -> io :: Result < super :: UCred > { backend :: net :: sockopt :: socket_peercred (fd . as_fd ()) }
    };
}

socket_peercred!();