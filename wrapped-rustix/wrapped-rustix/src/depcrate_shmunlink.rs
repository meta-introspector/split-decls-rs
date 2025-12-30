// Generated macro for unlink (function)
macro_rules! Depcrate_shmunlink {
() => {
// Module: crate::shm
// Provides: {"unlink"}
// Dependencies: {}
# [doc = " `shm_unlink(name)`—Unlinks a shared memory object."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/shm_unlink.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/shm_unlink.3.html"] # [doc (alias = "shm_unlink")] # [inline] pub fn unlink < P : path :: Arg > (name : P) -> io :: Result < () > { name . into_with_c_str (backend :: shm :: syscalls :: shm_unlink) }
};
}
