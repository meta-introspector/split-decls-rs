// Generated macro for wrap_serialize_field_with (function)
macro_rules! Depcrate_serwrap_serialize_field_with {
() => {
// Module: crate::ser
// Provides: {"wrap_serialize_field_with"}
// Dependencies: {}
fn wrap_serialize_field_with (params : & Parameters , field_ty : & syn :: Type , serialize_with : & syn :: ExprPath , field_expr : & TokenStream ,) -> TokenStream { wrap_serialize_with (params , serialize_with , & [field_ty] , & [quote ! (# field_expr)]) }
};
}
