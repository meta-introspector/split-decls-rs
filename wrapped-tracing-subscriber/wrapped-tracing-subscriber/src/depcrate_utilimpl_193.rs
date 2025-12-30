// Generated macro for impl_193 (impl)
macro_rules! Depcrate_utilimpl_193 {
() => {
// Module: crate::util
// Provides: {"impl_193"}
// Dependencies: {}
impl TryInitError { # [cfg (feature = "std")] fn new (e : impl Into < Box < dyn Error + Send + Sync + 'static > >) -> Self { Self { inner : e . into () } } # [cfg (not (feature = "std"))] fn new < T > (_ : T) -> Self { Self { _p : () } } }
};
}
