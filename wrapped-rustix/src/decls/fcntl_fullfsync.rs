macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fcntl_fullfsync {
    () => {
        deps!();
        # [doc = " `fcntl(fd, F_FULLFSYNC)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html"] # [doc (alias = "F_FULLSYNC")] # [inline] pub fn fcntl_fullfsync < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: fs :: syscalls :: fcntl_fullfsync (fd . as_fd ()) }
    };
}

fcntl_fullfsync!();