// Generated macro for set_thread_res_uid (function)
macro_rules! Depcrate_thread_idset_thread_res_uid {
() => {
// Module: crate::thread::id
// Provides: {"set_thread_res_uid"}
// Dependencies: {}
# [doc = " `setresuid(ruid, euid, suid)`—Sets the real, effective, and saved user ID"] # [doc = " of the calling thread."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This is not the `setresuid` you are looking for… POSIX requires uids to be"] # [doc = " process granular, but on Linux they are per-thread. Thus, this call only"] # [doc = " changes the uid for the current *thread*, not the entire process even"] # [doc = " though that is in violation of the POSIX standard."] # [doc = ""] # [doc = " For details on this distinction, see the C library vs. kernel differences"] # [doc = " in the [manual page][linux_notes] and the notes in [`set_thread_uid`]. This"] # [doc = " call implements the kernel behavior."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setresuid.2.html"] # [doc = " [linux_notes]: https://man7.org/linux/man-pages/man2/setresuid.2.html#NOTES"] # [inline] pub fn set_thread_res_uid < R , E , S > (ruid : R , euid : E , suid : S) -> io :: Result < () > where R : Into < Option < Uid > > , E : Into < Option < Uid > > , S : Into < Option < Uid > > , { backend :: thread :: syscalls :: setresuid_thread (ruid . into () , euid . into () , suid . into ()) }
};
}
