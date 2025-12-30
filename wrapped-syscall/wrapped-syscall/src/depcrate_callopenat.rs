// Generated macro for openat (function)
macro_rules! Depcrate_callopenat {
() => {
// Module: crate::call
// Provides: {"openat"}
// Dependencies: {}
# [doc = " Open a file at a specific path"] pub fn openat < T : AsRef < str > > (fd : usize , path : T , flags : usize , fcntl_flags : usize ,) -> Result < usize > { let path = path . as_ref () ; unsafe { syscall5 (SYS_OPENAT , fd , path . as_ptr () as usize , path . len () , flags , fcntl_flags ,) } }
};
}
