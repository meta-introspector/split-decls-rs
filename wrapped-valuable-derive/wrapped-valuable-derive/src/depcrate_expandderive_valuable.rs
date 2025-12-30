// Generated macro for derive_valuable (function)
macro_rules! Depcrate_expandderive_valuable {
() => {
// Module: crate::expand
// Provides: {"derive_valuable"}
// Dependencies: {}
pub (crate) fn derive_valuable (input : & mut syn :: DeriveInput) -> TokenStream { let cx = Context :: default () ; match & input . data { syn :: Data :: Struct (data) => derive_struct (cx , input , data) , syn :: Data :: Enum (data) => derive_enum (cx , input , data) , syn :: Data :: Union (data) => { Err (Error :: new (data . union_token . span , "#[derive(Valuable)] may only be used on structs and enums" ,)) } } . unwrap_or_else (Error :: into_compile_error) }
};
}
