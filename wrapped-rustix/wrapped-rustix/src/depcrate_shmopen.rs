// Generated macro for open (function)
macro_rules! Depcrate_shmopen {
() => {
// Module: crate::shm
// Provides: {"open"}
// Dependencies: {}
# [doc = " `shm_open(name, oflags, mode)`—Opens a shared memory object."] # [doc = ""] # [doc = " For portability, `name` should begin with a slash, contain no other"] # [doc = " slashes, and be no longer than an implementation-defined limit (255 on"] # [doc = " Linux)."] # [doc = ""] # [doc = " Exactly one of [`shm::OFlags::RDONLY`] and [`shm::OFlags::RDWR`] should be"] # [doc = " passed. The file descriptor will be opened with `FD_CLOEXEC` set."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/shm_open.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/shm_open.3.html"] # [doc (alias = "shm_open")] # [inline] pub fn open < P : path :: Arg > (name : P , flags : shm :: OFlags , mode : Mode) -> io :: Result < OwnedFd > { name . into_with_c_str (| name | backend :: shm :: syscalls :: shm_open (name , flags , mode)) }
};
}
