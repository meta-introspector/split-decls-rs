// Generated macro for impl_1299 (impl)
macro_rules! Depcrate_typesimpl_1299 {
() => {
// Module: crate::types
// Provides: {"impl_1299"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for InvalidAtomicOrdering { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { Self :: check_atomic_load_store (cx , expr) ; Self :: check_memory_fence (cx , expr) ; Self :: check_atomic_compare_exchange (cx , expr) ; } }
};
}
