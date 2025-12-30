// Generated macro for attr (function)
macro_rules! Depcrateattr {
() => {
// Module: crate
// Provides: {"attr"}
// Dependencies: {}
# [proc_macro_attribute] pub fn attr (args : TokenStream , input : TokenStream) -> TokenStream { attr :: parse (args) . and_then (| args | expand :: try_attr (args , input)) . unwrap_or_else (Error :: into_compile_error) }
};
}
