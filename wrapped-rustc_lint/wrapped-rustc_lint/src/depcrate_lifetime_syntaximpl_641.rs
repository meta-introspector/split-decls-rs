// Generated macro for impl_641 (impl)
macro_rules! Depcrate_lifetime_syntaximpl_641 {
() => {
// Module: crate::lifetime_syntax
// Provides: {"impl_641"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for LifetimeSyntax { # [instrument (skip_all)] fn check_fn (& mut self , cx : & LateContext < 'tcx > , _ : hir :: intravisit :: FnKind < 'tcx > , fd : & 'tcx hir :: FnDecl < 'tcx > , _ : & 'tcx hir :: Body < 'tcx > , _ : rustc_span :: Span , _ : rustc_span :: def_id :: LocalDefId ,) { check_fn_like (cx , fd) ; } # [instrument (skip_all)] fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , ti : & 'tcx hir :: TraitItem < 'tcx >) { match ti . kind { hir :: TraitItemKind :: Const (..) => { } hir :: TraitItemKind :: Fn (fn_sig , _trait_fn) => check_fn_like (cx , fn_sig . decl) , hir :: TraitItemKind :: Type (..) => { } } } # [instrument (skip_all)] fn check_foreign_item (& mut self , cx : & LateContext < 'tcx > , fi : & 'tcx rustc_hir :: ForeignItem < 'tcx > ,) { match fi . kind { hir :: ForeignItemKind :: Fn (fn_sig , _idents , _generics) => check_fn_like (cx , fn_sig . decl) , hir :: ForeignItemKind :: Static (..) => { } hir :: ForeignItemKind :: Type => { } } } }
};
}
