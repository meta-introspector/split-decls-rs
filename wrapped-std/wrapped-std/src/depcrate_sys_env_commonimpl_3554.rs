// Generated macro for impl_3554 (impl)
macro_rules! Depcrate_sys_env_commonimpl_3554 {
() => {
// Module: crate::sys::env::common
// Provides: {"impl_3554"}
// Dependencies: {}
impl fmt :: Debug for Env { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter . as_slice ()) . finish () } }
};
}
