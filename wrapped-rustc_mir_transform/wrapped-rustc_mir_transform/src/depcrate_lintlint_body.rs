// Generated macro for lint_body (function)
macro_rules! Depcrate_lintlint_body {
() => {
// Module: crate::lint
// Provides: {"lint_body"}
// Dependencies: {}
pub (super) fn lint_body < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , when : String) { let always_live_locals = & always_storage_live_locals (body) ; let maybe_storage_live = MaybeStorageLive :: new (Cow :: Borrowed (always_live_locals)) . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; let maybe_storage_dead = MaybeStorageDead :: new (Cow :: Borrowed (always_live_locals)) . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; let mut lint = Lint { tcx , when , body , is_fn_like : tcx . def_kind (body . source . def_id ()) . is_fn_like () , always_live_locals , maybe_storage_live , maybe_storage_dead , places : Default :: default () , } ; for (bb , data) in traversal :: reachable (body) { lint . visit_basic_block_data (bb , data) ; } }
};
}
