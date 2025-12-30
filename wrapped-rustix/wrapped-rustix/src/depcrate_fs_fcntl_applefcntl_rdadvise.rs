// Generated macro for fcntl_rdadvise (function)
macro_rules! Depcrate_fs_fcntl_applefcntl_rdadvise {
() => {
// Module: crate::fs::fcntl_apple
// Provides: {"fcntl_rdadvise"}
// Dependencies: {}
# [doc = " `fcntl(fd, F_RDADVISE, radvisory { offset, len })`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html"] # [doc (alias = "F_RDADVISE")] # [inline] pub fn fcntl_rdadvise < Fd : AsFd > (fd : Fd , offset : u64 , len : u64) -> io :: Result < () > { backend :: fs :: syscalls :: fcntl_rdadvise (fd . as_fd () , offset , len) }
};
}
