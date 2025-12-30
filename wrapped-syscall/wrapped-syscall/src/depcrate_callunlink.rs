// Generated macro for unlink (function)
macro_rules! Depcrate_callunlink {
() => {
// Module: crate::call
// Provides: {"unlink"}
// Dependencies: {}
# [doc = " Remove a file"] pub fn unlink < T : AsRef < str > > (path : T) -> Result < usize > { let path = path . as_ref () ; unsafe { syscall2 (SYS_UNLINK , path . as_ptr () as usize , path . len ()) } }
};
}
