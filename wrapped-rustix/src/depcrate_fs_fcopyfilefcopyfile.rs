// Generated macro for fcopyfile (function)
macro_rules! Depcrate_fs_fcopyfilefcopyfile {
() => {
// Module: crate::fs::fcopyfile
// Provides: {"fcopyfile"}
// Dependencies: {}
# [doc = " `fcopyfile(from, to, state, flags)`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `state` operand must be allocated with `copyfile_state_alloc` and not"] # [doc = " yet freed with `copyfile_state_free`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html"] # [inline] pub unsafe fn fcopyfile < FromFd : AsFd , ToFd : AsFd > (from : FromFd , to : ToFd , state : copyfile_state_t , flags : CopyfileFlags ,) -> io :: Result < () > { backend :: fs :: syscalls :: fcopyfile (from . as_fd () , to . as_fd () , state , flags) }
};
}
