macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fcntl_add_seals {
    () => {
        deps!();
        # [doc = " `fcntl(fd, F_ADD_SEALS)`—Add seals to `fd`'s inode."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "fuchsia"))] # [inline] # [doc (alias = "F_ADD_SEALS")] pub fn fcntl_add_seals < Fd : AsFd > (fd : Fd , seals : SealFlags) -> io :: Result < () > { backend :: fs :: syscalls :: fcntl_add_seals (fd . as_fd () , seals) }
    };
}

fcntl_add_seals!();