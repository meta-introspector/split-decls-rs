// Generated macro for impl_151 (impl)
macro_rules! Depcrate_schannel_credimpl_151 {
() => {
// Module: crate::schannel_cred
// Provides: {"impl_151"}
// Dependencies: {}
impl Drop for RawCredHandle { fn drop (& mut self) { unsafe { Identity :: FreeCredentialsHandle (& self . 0) ; } } }
};
}
