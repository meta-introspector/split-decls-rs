// Generated macro for nested_bodies_within (function)
macro_rules! Depcrate_nested_bodiesnested_bodies_within {
() => {
// Module: crate::nested_bodies
// Provides: {"nested_bodies_within"}
// Dependencies: {}
fn nested_bodies_within < 'tcx > (tcx : TyCtxt < 'tcx > , item : LocalDefId) -> & 'tcx ty :: List < LocalDefId > { let body = tcx . hir_body_owned_by (item) ; let mut collector = NestedBodiesVisitor { tcx , root_def_id : item . to_def_id () , nested_bodies : vec ! [] } ; collector . visit_body (body) ; tcx . mk_local_def_ids (& collector . nested_bodies) }
};
}
