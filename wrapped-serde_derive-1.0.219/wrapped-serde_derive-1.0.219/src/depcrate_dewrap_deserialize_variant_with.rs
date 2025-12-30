// Generated macro for wrap_deserialize_variant_with (function)
macro_rules! Depcrate_dewrap_deserialize_variant_with {
() => {
// Module: crate::de
// Provides: {"wrap_deserialize_variant_with"}
// Dependencies: {}
fn wrap_deserialize_variant_with (params : & Parameters , variant : & Variant , deserialize_with : & syn :: ExprPath ,) -> (TokenStream , TokenStream , TokenStream) { let field_tys = variant . fields . iter () . map (| field | field . ty) ; let (wrapper , wrapper_ty) = wrap_deserialize_with (params , & quote ! ((# (# field_tys) ,*)) , deserialize_with) ; let unwrap_fn = unwrap_to_variant_closure (params , variant , true) ; (wrapper , wrapper_ty , unwrap_fn) }
};
}
