// Generated macro for inner_mir_for_ctfe (function)
macro_rules! Depcrateinner_mir_for_ctfe {
() => {
// Module: crate
// Provides: {"inner_mir_for_ctfe"}
// Dependencies: {}
fn inner_mir_for_ctfe (tcx : TyCtxt < '_ > , def : LocalDefId) -> Body < '_ > { if tcx . is_constructor (def . to_def_id ()) { return shim :: build_adt_ctor (tcx , def . to_def_id ()) ; } let body = tcx . mir_drops_elaborated_and_const_checked (def) ; let body = match tcx . hir_body_const_context (def) { Some (hir :: ConstContext :: Const { .. } | hir :: ConstContext :: Static (_)) => body . steal () , Some (hir :: ConstContext :: ConstFn) => body . borrow () . clone () , None => bug ! ("`mir_for_ctfe` called on non-const {def:?}") , } ; let mut body = remap_mir_for_const_eval_select (tcx , body , hir :: Constness :: Const) ; pm :: run_passes (tcx , & mut body , & [& ctfe_limit :: CtfeLimit] , None , pm :: Optimizations :: Allowed) ; body }
};
}
