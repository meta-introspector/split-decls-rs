// Generated macro for impl_281 (impl)
macro_rules! Depcrate_impls_storage_livenessimpl_281 {
() => {
// Module: crate::impls::storage_liveness
// Provides: {"impl_281"}
// Dependencies: {}
impl < 'tcx > MaybeRequiresStorage < '_ , 'tcx > { # [doc = " Kill locals that are fully moved and have not been borrowed."] fn check_for_move (& mut self , state : & mut < Self as Analysis < 'tcx > > :: Domain , loc : Location) { let body = self . borrowed_locals . body () ; let mut visitor = MoveVisitor { state , borrowed_locals : & mut self . borrowed_locals } ; visitor . visit_location (body , loc) ; } }
};
}
