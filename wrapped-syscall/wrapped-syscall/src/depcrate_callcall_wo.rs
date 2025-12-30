// Generated macro for call_wo (function)
macro_rules! Depcrate_callcall_wo {
() => {
// Module: crate::call
// Provides: {"call_wo"}
// Dependencies: {}
# [doc = " SYS_CALL interface, write-only variant"] pub fn call_wo (fd : usize , payload : & [u8] , flags : CallFlags , metadata : & [u64]) -> Result < usize > { let combined_flags = flags | CallFlags :: WRITE ; unsafe { syscall5 (SYS_CALL , fd , payload . as_ptr () as * mut u8 as usize , payload . len () , metadata . len () | combined_flags . bits () , metadata . as_ptr () as usize ,) } }
};
}
