// Generated macro for assoc_fn_of_type (function)
macro_rules! Depcrate_mono_checks_move_checkassoc_fn_of_type {
() => {
// Module: crate::mono_checks::move_check
// Provides: {"assoc_fn_of_type"}
// Dependencies: {}
fn assoc_fn_of_type < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , fn_ident : Ident) -> Option < DefId > { for impl_def_id in tcx . inherent_impls (def_id) { if let Some (new) = tcx . associated_items (impl_def_id) . find_by_ident_and_kind (tcx , fn_ident , AssocTag :: Fn , def_id ,) { return Some (new . def_id) ; } } None }
};
}
