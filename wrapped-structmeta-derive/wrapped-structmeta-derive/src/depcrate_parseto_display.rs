// Generated macro for to_display (function)
macro_rules! Depcrate_parseto_display {
() => {
// Module: crate::parse
// Provides: {"to_display"}
// Dependencies: {}
fn to_display (index : usize , ident : & Option < Ident >) -> String { if let Some (ident) = ident { format ! ("{ident}") } else { format ! ("{index}") } }
};
}
