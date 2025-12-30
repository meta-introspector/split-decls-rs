// Generated macro for impl_3346 (impl)
macro_rules! Depcrate_sync_lazy_lockimpl_3346 {
() => {
// Module: crate::sync::lazy_lock
// Provides: {"impl_3346"}
// Dependencies: {}
# [stable (feature = "lazy_cell" , since = "1.80.0")] impl < T : fmt :: Debug , F > fmt :: Debug for LazyLock < T , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_tuple ("LazyLock") ; match LazyLock :: get (self) { Some (v) => d . field (v) , None => d . field (& format_args ! ("<uninit>")) , } ; d . finish () } }
};
}
