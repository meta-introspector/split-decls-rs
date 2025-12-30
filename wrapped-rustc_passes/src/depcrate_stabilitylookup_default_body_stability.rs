// Generated macro for lookup_default_body_stability (function)
macro_rules! Depcrate_stabilitylookup_default_body_stability {
() => {
// Module: crate::stability
// Provides: {"lookup_default_body_stability"}
// Dependencies: {}
# [instrument (level = "debug" , skip (tcx))] fn lookup_default_body_stability (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> Option < DefaultBodyStability > { if ! tcx . features () . staged_api () { return None ; } let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; find_attr ! (attrs , AttributeKind :: BodyStability { stability , .. } => * stability) }
};
}
