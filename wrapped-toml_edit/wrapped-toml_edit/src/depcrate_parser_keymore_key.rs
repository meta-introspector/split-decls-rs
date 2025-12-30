// Generated macro for more_key (function)
macro_rules! Depcrate_parser_keymore_key {
() => {
// Module: crate::parser::key
// Provides: {"more_key"}
// Dependencies: {}
fn more_key (input : & Input < '_ >) -> bool { let first = input . get (0) . map (| e | e . kind ()) ; let second = input . get (1) . map (| e | e . kind ()) ; if first == Some (EventKind :: KeySep) { true } else if first == Some (EventKind :: Whitespace) && second == Some (EventKind :: KeySep) { true } else { false } }
};
}
