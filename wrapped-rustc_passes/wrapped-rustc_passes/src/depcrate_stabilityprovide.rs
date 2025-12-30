// Generated macro for provide (function)
macro_rules! Depcrate_stabilityprovide {
() => {
// Module: crate::stability
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { check_mod_unstable_api_usage , stability_implications , lookup_stability , lookup_const_stability , lookup_default_body_stability , lookup_deprecation_entry , .. * providers } ; }
};
}
