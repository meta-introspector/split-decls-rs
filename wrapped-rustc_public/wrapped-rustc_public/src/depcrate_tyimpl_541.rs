// Generated macro for impl_541 (impl)
macro_rules! Depcrate_tyimpl_541 {
() => {
// Module: crate::ty
// Provides: {"impl_541"}
// Dependencies: {}
impl Binder < ExistentialTraitRef > { pub fn with_self_ty (& self , self_ty : Ty) -> Binder < TraitRef > { self . map_bound_ref (| trait_ref | trait_ref . with_self_ty (self_ty)) } }
};
}
