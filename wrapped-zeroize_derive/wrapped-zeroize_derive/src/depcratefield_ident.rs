// Generated macro for field_ident (function)
macro_rules! Depcratefield_ident {
() => {
// Module: crate
// Provides: {"field_ident"}
// Dependencies: {}
fn field_ident (n : usize , field : & Field) -> Ident { if let Some (ref name) = field . ident { name . clone () } else { format_ident ! ("__zeroize_field_{}" , n) } }
};
}
