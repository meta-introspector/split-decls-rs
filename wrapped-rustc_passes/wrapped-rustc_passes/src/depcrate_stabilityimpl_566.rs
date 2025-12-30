// Generated macro for impl_566 (impl)
macro_rules! Depcrate_stabilityimpl_566 {
() => {
// Module: crate::stability
// Provides: {"impl_566"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for CheckTraitImplStable < 'tcx > { fn visit_path (& mut self , path : & hir :: Path < 'tcx > , _id : hir :: HirId) { if let Some (def_id) = path . res . opt_def_id () && let Some (stab) = self . tcx . lookup_stability (def_id) { self . fully_stable &= stab . level . is_stable () ; } intravisit :: walk_path (self , path) } fn visit_trait_ref (& mut self , t : & 'tcx TraitRef < 'tcx >) { if let Res :: Def (DefKind :: Trait , trait_did) = t . path . res { if let Some (stab) = self . tcx . lookup_stability (trait_did) { self . fully_stable &= stab . level . is_stable () ; } } intravisit :: walk_trait_ref (self , t) } fn visit_ty (& mut self , t : & 'tcx Ty < 'tcx , AmbigArg >) { if let TyKind :: Never = t . kind { self . fully_stable = false ; } if let TyKind :: FnPtr (function) = t . kind { if extern_abi_stability (function . abi) . is_err () { self . fully_stable = false ; } } intravisit :: walk_ty (self , t) } fn visit_fn_decl (& mut self , fd : & 'tcx hir :: FnDecl < 'tcx >) { for ty in fd . inputs { self . visit_ty_unambig (ty) } if let hir :: FnRetTy :: Return (output_ty) = fd . output { match output_ty . kind { TyKind :: Never => { } _ => self . visit_ty_unambig (output_ty) , } } } }
};
}
