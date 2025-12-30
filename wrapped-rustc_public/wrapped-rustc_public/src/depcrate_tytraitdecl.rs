// Generated macro for TraitDecl (struct)
macro_rules! Depcrate_tyTraitDecl {
() => {
// Module: crate::ty
// Provides: {"TraitDecl"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct TraitDecl { pub def_id : TraitDef , pub safety : Safety , pub paren_sugar : bool , pub has_auto_impl : bool , pub is_marker : bool , pub is_coinductive : bool , pub skip_array_during_method_dispatch : bool , pub skip_boxed_slice_during_method_dispatch : bool , pub specialization_kind : TraitSpecializationKind , pub must_implement_one_of : Option < Vec < Ident > > , pub implement_via_object : bool , pub deny_explicit_impl : bool , }
};
}
