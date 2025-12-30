// Generated macro for impl_105 (impl)
macro_rules! Depcrate_errorimpl_105 {
() => {
// Module: crate::error
// Provides: {"impl_105"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < C , I , E : Send + Sync + 'static > FromExternalError < I , E > for ContextError < C > { # [inline] fn from_external_error (_input : & I , _e : E) -> Self { let err = Self :: new () ; err } }
};
}
