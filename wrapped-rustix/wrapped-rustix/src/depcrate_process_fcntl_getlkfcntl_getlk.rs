// Generated macro for fcntl_getlk (function)
macro_rules! Depcrate_process_fcntl_getlkfcntl_getlk {
() => {
// Module: crate::process::fcntl_getlk
// Provides: {"fcntl_getlk"}
// Dependencies: {}
# [doc = " `fcntl(fd, F_GETLK)`—Get the first lock that blocks the lock description"] # [doc = " pointed to by the argument `lock`. If no such lock is found, then `None` is"] # [doc = " returned."] # [doc = ""] # [doc = " If `lock.typ` is set to `FlockType::Unlocked`, the returned value/error is"] # [doc = " not explicitly defined, as per POSIX, and will depend on the underlying"] # [doc = " platform implementation."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fcntl.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [inline] # [doc (alias = "F_GETLK")] pub fn fcntl_getlk < Fd : AsFd > (fd : Fd , lock : & Flock) -> io :: Result < Option < Flock > > { backend :: process :: syscalls :: fcntl_getlk (fd . as_fd () , lock) }
};
}
