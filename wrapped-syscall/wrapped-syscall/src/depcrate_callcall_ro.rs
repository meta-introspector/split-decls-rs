// Generated macro for call_ro (function)
macro_rules! Depcrate_callcall_ro {
() => {
// Module: crate::call
// Provides: {"call_ro"}
// Dependencies: {}
# [doc = " SYS_CALL interface, read-only variant"] pub fn call_ro (fd : usize , payload : & mut [u8] , flags : CallFlags , metadata : & [u64]) -> Result < usize > { let combined_flags = flags | CallFlags :: READ ; unsafe { syscall5 (SYS_CALL , fd , payload . as_mut_ptr () as usize , payload . len () , metadata . len () | combined_flags . bits () , metadata . as_ptr () as usize ,) } }
};
}
