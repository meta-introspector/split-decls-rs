// Generated macro for try_lend (function)
macro_rules! Depcrate_os_xous_ffitry_lend {
() => {
// Module: crate::os::xous::ffi
// Provides: {"try_lend"}
// Dependencies: {}
pub (crate) fn try_lend (connection : Connection , opcode : usize , data : & [u8] , arg1 : usize , arg2 : usize ,) -> Result < (usize , usize) , Error > { lend_impl (connection , opcode , data , arg1 , arg2 , false) }
};
}
