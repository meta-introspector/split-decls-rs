// Generated macro for to_var_ident (function)
macro_rules! Depcrate_parseto_var_ident {
() => {
// Module: crate::parse
// Provides: {"to_var_ident"}
// Dependencies: {}
fn to_var_ident (index : usize , ident : & Option < Ident >) -> Ident { if let Some (ident) = ident { format_ident ! ("_{}" , ident) } else { format_ident ! ("_{}" , index) } }
};
}
