// Generated macro for static_visibility (function)
macro_rules! Depcrate_partitioningstatic_visibility {
() => {
// Module: crate::partitioning
// Provides: {"static_visibility"}
// Dependencies: {}
fn static_visibility < 'tcx > (tcx : TyCtxt < 'tcx > , can_be_internalized : & mut bool , def_id : DefId ,) -> Visibility { if tcx . is_reachable_non_generic (def_id) { * can_be_internalized = false ; default_visibility (tcx , def_id , false) } else { Visibility :: Hidden } }
};
}
