// Generated macro for impl_331 (impl)
macro_rules! Depcrate_http3_settingsimpl_331 {
() => {
// Module: crate::http3::settings
// Provides: {"impl_331"}
// Dependencies: {}
impl TimeoutCheckResult { fn set_expired (& mut self , typ : Http3TimeoutType) -> bool { use Http3TimeoutType :: * ; let field = match typ { PostAccept => & mut self . connection_timed_out , } ; * field = true ; true } }
};
}
