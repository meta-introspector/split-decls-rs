// Generated macro for macro_188 (macro)
macro_rules! Depcrate_macmacro_188 {
() => {
// Module: crate::mac
// Provides: {"macro_188"}
// Dependencies: {}
ast_struct ! { # [doc = " Represents a macro invocation. The Path indicates which macro"] # [doc = " is being invoked, and the vector of token-trees contains the source"] # [doc = " of the macro invocation."] pub struct Mac { pub path : Path , pub bang_token : tokens :: Bang , # [doc = " The `example` in `macro_rules! example { ... }`."] pub ident : Option < Ident >, pub tokens : Vec < TokenTree >, } }
};
}
