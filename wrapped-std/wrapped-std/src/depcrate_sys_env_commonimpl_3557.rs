// Generated macro for impl_3557 (impl)
macro_rules! Depcrate_sys_env_commonimpl_3557 {
() => {
// Module: crate::sys::env::common
// Provides: {"impl_3557"}
// Dependencies: {}
impl Iterator for Env { type Item = (OsString , OsString) ; fn next (& mut self) -> Option < (OsString , OsString) > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
