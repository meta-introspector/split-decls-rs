// Generated macro for lend_mut (function)
macro_rules! Depcrate_os_xous_ffilend_mut {
() => {
// Module: crate::os::xous::ffi
// Provides: {"lend_mut"}
// Dependencies: {}
pub (crate) fn lend_mut (connection : Connection , opcode : usize , data : & mut [u8] , arg1 : usize , arg2 : usize ,) -> Result < (usize , usize) , Error > { lend_mut_impl (connection , opcode , data , arg1 , arg2 , true) }
};
}
