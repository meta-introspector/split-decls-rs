// Generated macro for to_var_ident (function)
macro_rules! Depcrate_to_tokensto_var_ident {
() => {
// Module: crate::to_tokens
// Provides: {"to_var_ident"}
// Dependencies: {}
fn to_var_ident (index : Option < usize > , ident : & Option < Ident >) -> Ident { if let Some (ident) = ident { format_ident ! ("_{}" , ident) } else { format_ident ! ("_{}" , index . unwrap ()) } }
};
}
