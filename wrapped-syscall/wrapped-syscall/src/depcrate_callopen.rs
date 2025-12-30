// Generated macro for open (function)
macro_rules! Depcrate_callopen {
() => {
// Module: crate::call
// Provides: {"open"}
// Dependencies: {}
# [doc = " Open a file"] pub fn open < T : AsRef < str > > (path : T , flags : usize) -> Result < usize > { let path = path . as_ref () ; unsafe { syscall3 (SYS_OPEN , path . as_ptr () as usize , path . len () , flags) } }
};
}
