// Generated macro for impl_200 (impl)
macro_rules! Depcrate_nested_bodiesimpl_200 {
() => {
// Module: crate::nested_bodies
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for NestedBodiesVisitor < 'tcx > { fn visit_nested_body (& mut self , id : hir :: BodyId) { let body_def_id = self . tcx . hir_body_owner_def_id (id) ; if self . tcx . typeck_root_def_id (body_def_id . to_def_id ()) == self . root_def_id { let body = self . tcx . hir_body (id) ; self . visit_body (body) ; self . nested_bodies . push (body_def_id) ; } } }
};
}
