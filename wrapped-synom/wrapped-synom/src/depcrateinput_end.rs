// Generated macro for input_end (function)
macro_rules! Depcrateinput_end {
() => {
// Module: crate
// Provides: {"input_end"}
// Dependencies: {}
# [doc (hidden)] pub fn input_end (input : Cursor) -> PResult < 'static , & 'static str > { if input . eof () { Ok ((Cursor :: empty () , "")) } else { parse_error () } }
};
}
