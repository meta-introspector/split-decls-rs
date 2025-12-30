// Generated macro for impl_72 (impl)
macro_rules! Depcrate_struct_metaimpl_72 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_72"}
// Dependencies: {}
impl ArgKind { fn to_helper_name_index_variant (self) -> TokenStream { match self { Self :: Flag => quote ! (Flag) , Self :: NameValue => quote ! (NameValue) , Self :: NameArgs => quote ! (NameArgs) , } } }
};
}
