// Generated macro for styled (function)
macro_rules! Depcratestyled {
() => {
// Module: crate
// Provides: {"styled"}
// Dependencies: {}
fn styled (ansi : bool , style : Style , text : impl AsRef < str >) -> String { if ansi { style . paint (text . as_ref ()) . to_string () } else { text . as_ref () . to_string () } }
};
}
