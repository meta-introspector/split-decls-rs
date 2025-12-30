// Generated macro for impl_104 (impl)
macro_rules! Depcrate_errorimpl_104 {
() => {
// Module: crate::error
// Provides: {"impl_104"}
// Dependencies: {}
# [cfg (feature = "std")] impl < C , I , E : std :: error :: Error + Send + Sync + 'static > FromExternalError < I , E > for ContextError < C > { # [inline] fn from_external_error (_input : & I , e : E) -> Self { let mut err = Self :: new () ; { err . cause = Some (Box :: new (e)) ; } err } }
};
}
