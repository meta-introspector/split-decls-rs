// Generated macro for copyfile_state_free (function)
macro_rules! Depcrate_fs_fcopyfilecopyfile_state_free {
() => {
// Module: crate::fs::fcopyfile
// Provides: {"copyfile_state_free"}
// Dependencies: {}
# [doc = " `copyfile_state_free(state)`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `state` operand must be allocated with `copyfile_state_alloc` and not"] # [doc = " yet freed with `copyfile_state_free`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html"] # [inline] pub unsafe fn copyfile_state_free (state : copyfile_state_t) -> io :: Result < () > { backend :: fs :: syscalls :: copyfile_state_free (state) }
};
}
