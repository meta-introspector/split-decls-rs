// Generated macro for validate_trait_object_ptr_ty (function)
macro_rules! Depcrate_validationvalidate_trait_object_ptr_ty {
() => {
// Module: crate::validation
// Provides: {"validate_trait_object_ptr_ty"}
// Dependencies: {}
fn validate_trait_object_ptr_ty (ty : ast :: PtrType , errors : & mut Vec < SyntaxError >) { match ty . ty () { Some (ast :: Type :: DynTraitType (ty)) => { if let Some (err) = validate_trait_object_ty_plus (ty) { errors . push (err) ; } } Some (ast :: Type :: ImplTraitType (ty)) => { if let Some (err) = validate_impl_object_ty_plus (ty) { errors . push (err) ; } } _ => () , } }
};
}
