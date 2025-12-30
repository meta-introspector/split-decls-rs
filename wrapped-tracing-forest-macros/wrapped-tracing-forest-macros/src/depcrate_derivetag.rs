// Generated macro for tag (function)
macro_rules! Depcrate_derivetag {
() => {
// Module: crate::derive
// Provides: {"tag"}
// Dependencies: {}
pub fn tag (input : TokenStream) -> TokenStream { let input = syn :: parse_macro_input ! (input as syn :: DeriveInput) ; if let syn :: Visibility :: Restricted (vis) = & input . vis { if syn :: parse_str :: < syn :: Path > ("crate") . expect ("Parsing failure") == * vis . path { let res = match & input . data { syn :: Data :: Struct (data) => impl_struct (data , & input) , syn :: Data :: Enum (data) => impl_enum (data , & input) , syn :: Data :: Union (_) => Err (syn :: Error :: new_spanned (input , "union tags are not supported" ,)) , } ; return res . unwrap_or_else (| err | err . to_compile_error ()) . into () ; } } syn :: Error :: new_spanned (input . vis , "must be visible in the crate, use `pub(crate)`") . to_compile_error () . into () }
};
}
