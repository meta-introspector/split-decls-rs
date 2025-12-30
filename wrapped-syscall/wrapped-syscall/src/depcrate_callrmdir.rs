// Generated macro for rmdir (function)
macro_rules! Depcrate_callrmdir {
() => {
// Module: crate::call
// Provides: {"rmdir"}
// Dependencies: {}
# [doc = " Remove a directory"] pub fn rmdir < T : AsRef < str > > (path : T) -> Result < usize > { let path = path . as_ref () ; unsafe { syscall2 (SYS_RMDIR , path . as_ptr () as usize , path . len ()) } }
};
}
