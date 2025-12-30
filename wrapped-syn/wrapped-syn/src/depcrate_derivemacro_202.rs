// Generated macro for macro_202 (macro)
macro_rules! Depcrate_derivemacro_202 {
() => {
// Module: crate::derive
// Provides: {"macro_202"}
// Dependencies: {}
ast_struct ! { # [doc = " Struct or enum sent to a `proc_macro_derive` macro."] pub struct DeriveInput { # [doc = " Name of the struct or enum."] pub ident : Ident , # [doc = " Visibility of the struct or enum."] pub vis : Visibility , # [doc = " Attributes tagged on the whole struct or enum."] pub attrs : Vec < Attribute >, # [doc = " Generics required to complete the definition."] pub generics : Generics , # [doc = " Data within the struct or enum."] pub body : Body , } }
};
}
