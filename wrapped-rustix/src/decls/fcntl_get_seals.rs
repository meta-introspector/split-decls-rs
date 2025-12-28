macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fcntl_get_seals {
    () => {
        deps!();
        # [doc = " `fcntl(fd, F_GET_SEALS)`—Return the seals for `fd`'s inode."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "fuchsia"))] # [inline] # [doc (alias = "F_GET_SEALS")] pub fn fcntl_get_seals < Fd : AsFd > (fd : Fd) -> io :: Result < SealFlags > { backend :: fs :: syscalls :: fcntl_get_seals (fd . as_fd ()) }
    };
}

fcntl_get_seals!()