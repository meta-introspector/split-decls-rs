// Generated macro for impl_3365 (impl)
macro_rules! Depcrate_sync_once_lockimpl_3365 {
() => {
// Module: crate::sync::once_lock
// Provides: {"impl_3365"}
// Dependencies: {}
# [stable (feature = "once_cell" , since = "1.70.0")] impl < T : fmt :: Debug > fmt :: Debug for OnceLock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_tuple ("OnceLock") ; match self . get () { Some (v) => d . field (v) , None => d . field (& format_args ! ("<uninit>")) , } ; d . finish () } }
};
}
