// Generated macro for set_thread_groups (function)
macro_rules! Depcrate_thread_idset_thread_groups {
() => {
// Module: crate::thread::id
// Provides: {"set_thread_groups"}
// Dependencies: {}
# [doc = " `setgroups(groups)`—Sets the supplementary group IDs for the calling"] # [doc = " thread."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This is not the `setgroups` you are looking for… POSIX requires gids to be"] # [doc = " process granular, but on Linux they are per-thread. Thus, this call only"] # [doc = " changes the gids for the current *thread*, not the entire process even"] # [doc = " though that is in violation of the POSIX standard."] # [doc = ""] # [doc = " For details on this distinction, see the C library vs. kernel differences"] # [doc = " in the [manual page][linux_notes]. This call implements the kernel"] # [doc = " behavior."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setgroups.2.html"] # [doc = " [linux_notes]: https://man7.org/linux/man-pages/man2/setgroups.2.html#NOTES"] # [cfg (linux_kernel)] # [inline] pub fn set_thread_groups (groups : & [Gid]) -> io :: Result < () > { backend :: thread :: syscalls :: setgroups_thread (groups) }
};
}
