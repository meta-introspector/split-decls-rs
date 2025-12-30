// Generated macro for supertype (function)
macro_rules! Depcrate_supertypesupertype {
() => {
// Module: crate::supertype
// Provides: {"supertype"}
// Dependencies: {}
# [doc = " The implementation of the `supertype` macro."] # [doc = ""] # [doc = " For an entity enum `Foo` with variants `Variant1, ..., VariantN`, we generate"] # [doc = " mappings between the variants and their corresponding supertypes."] pub (crate) fn supertype (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let enum_item = parse_macro_input ! (input as syn :: ItemEnum) ; match enum_impl (enum_item) { Ok (v) => v . into () , Err (e) => token_stream_with_error (input , e) , } }
};
}
