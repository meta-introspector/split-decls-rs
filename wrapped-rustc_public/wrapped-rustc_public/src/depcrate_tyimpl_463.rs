// Generated macro for impl_463 (impl)
macro_rules! Depcrate_tyimpl_463 {
() => {
// Module: crate::ty
// Provides: {"impl_463"}
// Dependencies: {}
impl Span { # [doc = " Return filename for diagnostic purposes"] pub fn get_filename (& self) -> Filename { with (| c | c . get_filename (self)) } # [doc = " Return lines that correspond to this `Span`"] pub fn get_lines (& self) -> LineInfo { with (| c | c . get_lines (self)) } # [doc = " Return the span location to be printed in diagnostic messages."] # [doc = ""] # [doc = " This may leak local file paths and should not be used to build artifacts that may be"] # [doc = " distributed."] pub fn diagnostic (& self) -> String { with (| c | c . span_to_string (* self)) } }
};
}
