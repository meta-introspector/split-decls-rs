// Generated macro for lend (function)
macro_rules! Depcrate_os_xous_ffilend {
() => {
// Module: crate::os::xous::ffi
// Provides: {"lend"}
// Dependencies: {}
pub (crate) fn lend (connection : Connection , opcode : usize , data : & [u8] , arg1 : usize , arg2 : usize ,) -> Result < (usize , usize) , Error > { lend_impl (connection , opcode , data , arg1 , arg2 , true) }
};
}
