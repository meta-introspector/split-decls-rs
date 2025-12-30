// Generated macro for copyfile_state_get (function)
macro_rules! Depcrate_fs_fcopyfilecopyfile_state_get {
() => {
// Module: crate::fs::fcopyfile
// Provides: {"copyfile_state_get"}
// Dependencies: {}
# [doc = " `copyfile_state_get(state, flags, dst)`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `state` operand must be allocated with `copyfile_state_alloc` and not"] # [doc = " yet freed with `copyfile_state_free`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html"] # [inline] pub unsafe fn copyfile_state_get (state : copyfile_state_t , flag : u32 , dst : * mut core :: ffi :: c_void ,) -> io :: Result < () > { backend :: fs :: syscalls :: copyfile_state_get (state , flag , dst) }
};
}
