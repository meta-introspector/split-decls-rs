// Generated macro for impl_98 (impl)
macro_rules! Depcrate_inputimpl_98 {
() => {
// Module: crate::input
// Provides: {"impl_98"}
// Dependencies: {}
impl InputType { # [doc = " Optionally unwraps as a PredicateForm."] pub fn predicate_form (& self) -> Option < & PredicateForm > { match self { InputType :: PredicateForm (pf) => Some (pf) , _ => None , } } # [doc = " Optionally unwraps as a mutable PredicateForm"] pub fn predicate_form_mut (& mut self) -> Option < & mut PredicateForm > { match self { InputType :: PredicateForm (pf) => Some (pf) , _ => None , } } # [doc = " Optionally unwraps as a TypeKind."] pub fn typekind (& self) -> Option < & TypeKind > { match self { InputType :: Type (ty) => Some (ty) , _ => None , } } # [doc = " Optionally unwraps as a NVariantOp"] pub fn n_variant_op (& self) -> Option < & WildString > { match self { InputType :: NVariantOp (Some (op)) => Some (op) , _ => None , } } }
};
}
