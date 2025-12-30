// Generated macro for impl_58 (impl)
macro_rules! Depcrate_struct_metaimpl_58 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_58"}
// Dependencies: {}
impl NameFilter { fn to_code (self) -> TokenStream { match self { NameFilter :: None => quote ! (&| _ | true) , NameFilter :: SnakeCase => quote ! (&:: structmeta :: helpers :: is_snake_case) , } } }
};
}
