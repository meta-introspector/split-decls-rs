// Generated macro for impl_245 (impl)
macro_rules! Depcrate_lineimpl_245 {
() => {
// Module: crate::line
// Provides: {"impl_245"}
// Dependencies: {}
impl LineBreakType for Latin1 { fn get_linebreak_property_with_rule (iterator : & LineBreakIterator < Self > , c : u8) -> u8 { iterator . data . property_table . get32 (c as u32) } # [inline] fn use_complex_breaking (_iterator : & LineBreakIterator < Self > , _c : u8) -> bool { false } fn line_handle_complex_language (_ : & mut LineBreakIterator < Self > , _ : Self :: CharType ,) -> Option < usize > { unreachable ! () } }
};
}
