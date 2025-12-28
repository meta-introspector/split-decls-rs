macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fcntl_getpipe_size {
    () => {
        deps!();
        # [doc = " `fnctl(fd, F_GETPIPE_SZ)`—Return the buffer capacity of a pipe."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [cfg (linux_kernel)] # [inline] pub fn fcntl_getpipe_size < Fd : AsFd > (fd : Fd) -> io :: Result < usize > { backend :: pipe :: syscalls :: fcntl_getpipe_size (fd . as_fd ()) }
    };
}

fcntl_getpipe_size!();