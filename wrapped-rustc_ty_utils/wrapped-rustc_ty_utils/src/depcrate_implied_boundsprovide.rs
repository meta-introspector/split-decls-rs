// Generated macro for provide (function)
macro_rules! Depcrate_implied_boundsprovide {
() => {
// Module: crate::implied_bounds
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { assumed_wf_types , assumed_wf_types_for_rpitit : | tcx , def_id | { assert ! (tcx . is_impl_trait_in_trait (def_id . to_def_id ())) ; tcx . assumed_wf_types (def_id) } , .. * providers } ; }
};
}
