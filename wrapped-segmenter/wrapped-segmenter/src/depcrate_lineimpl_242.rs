// Generated macro for impl_242 (impl)
macro_rules! Depcrate_lineimpl_242 {
() => {
// Module: crate::line
// Provides: {"impl_242"}
// Dependencies: {}
impl LineBreakType for Utf8 { fn get_linebreak_property_with_rule (iterator : & LineBreakIterator < Self > , c : char) -> u8 { iterator . data . get_linebreak_property_utf32_with_rule (c as u32 , iterator . options . strictness , iterator . options . word_option ,) } # [inline] fn use_complex_breaking (iterator : & LineBreakIterator < Self > , c : char) -> bool { iterator . data . use_complex_breaking_utf32 (c as u32) } fn line_handle_complex_language (iter : & mut LineBreakIterator < '_ , '_ , Self > , left_codepoint : char ,) -> Option < usize > { line_handle_complex_language_utf8 (iter , left_codepoint) } }
};
}
