// Generated macro for impl_283 (impl)
macro_rules! Depcrate_impls_storage_livenessimpl_283 {
() => {
// Module: crate::impls::storage_liveness
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for MoveVisitor < '_ , '_ , 'tcx > { fn visit_local (& mut self , local : Local , context : PlaceContext , loc : Location) { if PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Move) == context { self . borrowed_locals . seek_before_primary_effect (loc) ; if ! self . borrowed_locals . get () . contains (local) { self . state . kill (local) ; } } } }
};
}
