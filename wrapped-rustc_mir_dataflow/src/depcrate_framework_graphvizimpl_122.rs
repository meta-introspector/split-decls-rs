// Generated macro for impl_122 (impl)
macro_rules! Depcrate_framework_graphvizimpl_122 {
() => {
// Module: crate::framework::graphviz
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'mir , 'tcx , A > Formatter < 'mir , 'tcx , A > where A : Analysis < 'tcx > , { fn new (body : & 'mir Body < 'tcx > , analysis : & 'mir mut A , results : & 'mir Results < A :: Domain > , style : OutputStyle ,) -> Self { let reachable = traversal :: reachable_as_bitset (body) ; Formatter { body , analysis : analysis . into () , results , style , reachable } } }
};
}
