// Generated macro for impl_3552 (impl)
macro_rules! Depcrate_sys_env_commonimpl_3552 {
() => {
// Module: crate::sys::env::common
// Provides: {"impl_3552"}
// Dependencies: {}
impl fmt :: Debug for EnvStrDebug < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . slice . iter () . map (| (a , b) | (a . to_str () . unwrap () , b . to_str () . unwrap ()))) . finish () } }
};
}
