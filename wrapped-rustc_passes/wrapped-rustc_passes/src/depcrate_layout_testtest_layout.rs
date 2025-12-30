// Generated macro for test_layout (function)
macro_rules! Depcrate_layout_testtest_layout {
() => {
// Module: crate::layout_test
// Provides: {"test_layout"}
// Dependencies: {}
pub fn test_layout (tcx : TyCtxt < '_ >) { if ! tcx . features () . rustc_attrs () { return ; } for id in tcx . hir_crate_items (()) . definitions () { for attr in tcx . get_attrs (id , sym :: rustc_layout) { match tcx . def_kind (id) { DefKind :: TyAlias | DefKind :: Enum | DefKind :: Struct | DefKind :: Union => { dump_layout_of (tcx , id , attr) ; } _ => { tcx . dcx () . emit_err (LayoutInvalidAttribute { span : tcx . def_span (id) }) ; } } } } }
};
}
