macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fcntl_rdadvise {
    () => {
        deps!();
        # [doc = " `fcntl(fd, F_RDADVISE, radvisory { offset, len })`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html"] # [doc (alias = "F_RDADVISE")] # [inline] pub fn fcntl_rdadvise < Fd : AsFd > (fd : Fd , offset : u64 , len : u64) -> io :: Result < () > { backend :: fs :: syscalls :: fcntl_rdadvise (fd . as_fd () , offset , len) }
    };
}

fcntl_rdadvise!()