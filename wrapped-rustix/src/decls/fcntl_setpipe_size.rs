macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fcntl_setpipe_size {
    () => {
        deps!();
        # [doc = " `fnctl(fd, F_SETPIPE_SZ)`—Set the buffer capacity of a pipe."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [cfg (linux_kernel)] # [inline] pub fn fcntl_setpipe_size < Fd : AsFd > (fd : Fd , size : usize) -> io :: Result < usize > { backend :: pipe :: syscalls :: fcntl_setpipe_size (fd . as_fd () , size) }
    };
}

fcntl_setpipe_size!();