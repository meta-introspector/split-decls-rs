// Generated macro for impl_3553 (impl)
macro_rules! Depcrate_sys_env_commonimpl_3553 {
() => {
// Module: crate::sys::env::common
// Provides: {"impl_3553"}
// Dependencies: {}
impl Env { pub (super) fn new (env : Vec < (OsString , OsString) >) -> Self { Env { iter : env . into_iter () } } pub fn str_debug (& self) -> impl fmt :: Debug + '_ { EnvStrDebug { slice : self . iter . as_slice () } } }
};
}
