// Generated macro for flink (function)
macro_rules! Depcrate_callflink {
() => {
// Module: crate::call
// Provides: {"flink"}
// Dependencies: {}
# [doc = " Create a link to a file"] pub fn flink < T : AsRef < str > > (fd : usize , path : T) -> Result < usize > { let path = path . as_ref () ; unsafe { syscall3 (SYS_FLINK , fd , path . as_ptr () as usize , path . len ()) } }
};
}
