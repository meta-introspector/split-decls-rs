// Generated macro for set_thread_gid (function)
macro_rules! Depcrate_thread_idset_thread_gid {
() => {
// Module: crate::thread::id
// Provides: {"set_thread_gid"}
// Dependencies: {}
# [doc = " `setgid(gid)`—Sets the effective group ID of the current thread."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This is not the `setgid` you are looking for… POSIX requires gids to be"] # [doc = " process granular, but on Linux they are per-thread. Thus, this call only"] # [doc = " changes the gid for the current *thread*, not the entire process even"] # [doc = " though that is in violation of the POSIX standard."] # [doc = ""] # [doc = " For details on this distinction, see the C library vs. kernel differences"] # [doc = " in the [manual page][linux_notes]. This call implements the kernel"] # [doc = " behavior."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/setgid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setgid.2.html"] # [doc = " [linux_notes]: https://man7.org/linux/man-pages/man2/setgid.2.html#NOTES"] # [inline] pub fn set_thread_gid (gid : Gid) -> io :: Result < () > { backend :: thread :: syscalls :: setgid_thread (gid) }
};
}
