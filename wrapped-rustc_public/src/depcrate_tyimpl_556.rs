// Generated macro for impl_556 (impl)
macro_rules! Depcrate_tyimpl_556 {
() => {
// Module: crate::ty
// Provides: {"impl_556"}
// Dependencies: {}
impl Binder < ExistentialTraitRef > { pub fn with_self_ty (& self , self_ty : Ty) -> Binder < TraitRef > { self . map_bound_ref (| trait_ref | trait_ref . with_self_ty (self_ty)) } }
};
}
