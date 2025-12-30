// Generated macro for NUL_ERR (const)
macro_rules! Depcrate_sys_pal_common_small_c_stringNUL_ERR {
() => {
// Module: crate::sys::pal::common::small_c_string
// Provides: {"NUL_ERR"}
// Dependencies: {}
const NUL_ERR : io :: Error = io :: const_error ! (io :: ErrorKind :: InvalidInput , "file name contained an unexpected NUL byte") ;
};
}
