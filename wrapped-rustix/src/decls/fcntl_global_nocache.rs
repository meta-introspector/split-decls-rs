macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fcntl_global_nocache {
    () => {
        deps!();
        # [doc = " `fcntl(fd, F_GLOBAL_NOCACHE, value)`—Turn data caching off or on for all"] # [doc = " file descriptors."] # [doc = ""] # [doc = " See [this mailing list post] for additional information about the meanings"] # [doc = " of `F_NOCACHE` and `F_GLOBAL_NOCACHE`."] # [doc = ""] # [doc = " [this mailing list post]: https://lists.apple.com/archives/filesystem-dev/2007/Sep/msg00010.html"] # [doc = ""] # [doc = " See also [`fcntl_nocache`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html"] # [doc (alias = "F_GLOBAL_NOCACHE")] # [inline] pub fn fcntl_global_nocache < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: fs :: syscalls :: fcntl_global_nocache (fd . as_fd () , value) }
    };
}

fcntl_global_nocache!()