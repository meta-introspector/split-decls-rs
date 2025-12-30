// Generated macro for call_rw (function)
macro_rules! Depcrate_callcall_rw {
() => {
// Module: crate::call
// Provides: {"call_rw"}
// Dependencies: {}
# [doc = " SYS_CALL interface, read-write variant"] pub fn call_rw (fd : usize , payload : & mut [u8] , flags : CallFlags , metadata : & [u64]) -> Result < usize > { let combined_flags = flags | CallFlags :: READ | CallFlags :: WRITE ; unsafe { syscall5 (SYS_CALL , fd , payload . as_mut_ptr () as usize , payload . len () , metadata . len () | combined_flags . bits () , metadata . as_ptr () as usize ,) } }
};
}
