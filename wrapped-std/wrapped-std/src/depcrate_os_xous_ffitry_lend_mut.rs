// Generated macro for try_lend_mut (function)
macro_rules! Depcrate_os_xous_ffitry_lend_mut {
() => {
// Module: crate::os::xous::ffi
// Provides: {"try_lend_mut"}
// Dependencies: {}
pub (crate) fn try_lend_mut (connection : Connection , opcode : usize , data : & mut [u8] , arg1 : usize , arg2 : usize ,) -> Result < (usize , usize) , Error > { lend_mut_impl (connection , opcode , data , arg1 , arg2 , false) }
};
}
