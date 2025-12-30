// Generated macro for wrap_deserialize_field_with (function)
macro_rules! Depcrate_dewrap_deserialize_field_with {
() => {
// Module: crate::de
// Provides: {"wrap_deserialize_field_with"}
// Dependencies: {}
fn wrap_deserialize_field_with (params : & Parameters , field_ty : & syn :: Type , deserialize_with : & syn :: ExprPath ,) -> (TokenStream , TokenStream) { wrap_deserialize_with (params , & quote ! (# field_ty) , deserialize_with) }
};
}
