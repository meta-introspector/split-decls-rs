// Generated macro for copyfile_state_alloc (function)
macro_rules! Depcrate_fs_fcopyfilecopyfile_state_alloc {
() => {
// Module: crate::fs::fcopyfile
// Provides: {"copyfile_state_alloc"}
// Dependencies: {}
# [doc = " `copyfile_state_alloc()`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html"] # [inline] pub fn copyfile_state_alloc () -> io :: Result < copyfile_state_t > { backend :: fs :: syscalls :: copyfile_state_alloc () }
};
}
