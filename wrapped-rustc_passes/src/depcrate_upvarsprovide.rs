// Generated macro for provide (function)
macro_rules! Depcrate_upvarsprovide {
() => {
// Module: crate::upvars
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (providers : & mut Providers) { providers . upvars_mentioned = | tcx , def_id | { if ! tcx . is_closure_like (def_id) { return None ; } let local_def_id = def_id . expect_local () ; let body = tcx . hir_maybe_body_owned_by (local_def_id) ? ; let mut local_collector = LocalCollector :: default () ; local_collector . visit_body (& body) ; let mut capture_collector = CaptureCollector { tcx , locals : & local_collector . locals , upvars : FxIndexMap :: default () , } ; capture_collector . visit_body (& body) ; if ! capture_collector . upvars . is_empty () { Some (tcx . arena . alloc (capture_collector . upvars)) } else { None } } ; }
};
}
