// Generated macro for impl_1315 (impl)
macro_rules! Depcrate_unqualified_local_importsimpl_1315 {
() => {
// Module: crate::unqualified_local_imports
// Provides: {"impl_1315"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnqualifiedLocalImports { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < 'tcx >) { let hir :: ItemKind :: Use (path , _kind) = item . kind else { return } ; let is_local_import = matches ! (path . res . type_ns , Some (hir :: def :: Res :: Def (_ , def_id)) if def_id . is_local ()) || matches ! (path . res . value_ns , Some (hir :: def :: Res :: Def (_ , def_id)) if def_id . is_local ()) ; if ! is_local_import { return ; } let Some (first_seg) = path . segments . first () else { return } ; if matches ! (first_seg . ident . name , kw :: SelfLower | kw :: Super | kw :: Crate) { return ; } let encl_item_id = cx . tcx . hir_get_parent_item (item . hir_id ()) ; let encl_item = cx . tcx . hir_node_by_def_id (encl_item_id . def_id) ; if encl_item . fn_kind () . is_some () { return ; } cx . emit_span_lint (UNQUALIFIED_LOCAL_IMPORTS , first_seg . ident . span , lints :: UnqualifiedLocalImportsDiag { } ,) ; } }
};
}
