// Generated macro for frename (function)
macro_rules! Depcrate_callfrename {
() => {
// Module: crate::call
// Provides: {"frename"}
// Dependencies: {}
# [doc = " Rename a file"] pub fn frename < T : AsRef < str > > (fd : usize , path : T) -> Result < usize > { let path = path . as_ref () ; unsafe { syscall3 (SYS_FRENAME , fd , path . as_ptr () as usize , path . len ()) } }
};
}
