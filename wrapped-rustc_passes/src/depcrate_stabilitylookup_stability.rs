// Generated macro for lookup_stability (function)
macro_rules! Depcrate_stabilitylookup_stability {
() => {
// Module: crate::stability
// Provides: {"lookup_stability"}
// Dependencies: {}
# [instrument (level = "debug" , skip (tcx))] fn lookup_stability (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < Stability > { if ! tcx . features () . staged_api () { if ! tcx . sess . opts . unstable_opts . force_unstable_if_unmarked { return None ; } let Some (parent) = tcx . opt_local_parent (def_id) else { return Some (FORCE_UNSTABLE) } ; if inherit_deprecation (tcx . def_kind (def_id)) { let parent = tcx . lookup_stability (parent) ? ; if parent . is_unstable () { return Some (parent) ; } } return None ; } let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; let stab = find_attr ! (attrs , AttributeKind :: Stability { stability , span : _ } => * stability) ; if let Some (stab) = stab { return Some (stab) ; } if inherit_deprecation (tcx . def_kind (def_id)) { let Some (parent) = tcx . opt_local_parent (def_id) else { return tcx . sess . opts . unstable_opts . force_unstable_if_unmarked . then_some (FORCE_UNSTABLE) ; } ; let parent = tcx . lookup_stability (parent) ? ; if parent . is_unstable () || inherit_stability (tcx . def_kind (def_id)) { return Some (parent) ; } } None }
};
}
