// Generated macro for delete_module (function)
macro_rules! Depcrate_systemdelete_module {
() => {
// Module: crate::system
// Provides: {"delete_module"}
// Dependencies: {}
# [doc = " `delete_module`—Unload a kernel module."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/delete_module.2.html"] # [inline] # [cfg (linux_kernel)] pub fn delete_module (name : & CStr , flags : c_int) -> io :: Result < () > { backend :: system :: syscalls :: delete_module (name , flags) }
};
}
