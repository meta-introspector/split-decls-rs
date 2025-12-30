// Generated macro for impl_299 (impl)
macro_rules! Depcrate_wordimpl_299 {
() => {
// Module: crate::word
// Provides: {"impl_299"}
// Dependencies: {}
impl WordBreakType for Utf16 { fn word_handle_complex_language (iter : & mut RuleBreakIterator < Self > , left_codepoint : Self :: CharType ,) -> Option < usize > { let start_iter = iter . iter . clone () ; let start_point = iter . current_pos_data ; let mut s = vec ! [left_codepoint as u16] ; loop { debug_assert ! (! iter . is_eof ()) ; s . push (iter . get_current_codepoint () ? as u16) ; iter . advance_iter () ; if let Some (current_break_property) = iter . get_current_break_property () { if current_break_property != iter . data . complex_property { break ; } } else { break ; } } iter . iter = start_iter ; iter . current_pos_data = start_point ; # [expect (clippy :: unwrap_used)] let breaks = iter . complex . unwrap () . complex_language_segment_utf16 (& s) ; iter . result_cache = breaks ; let first_pos = * iter . result_cache . first () ? ; let mut i = 1 ; loop { if i == first_pos { iter . result_cache = iter . result_cache . iter () . skip (1) . map (| r | r - i) . collect () ; return iter . get_current_position () ; } debug_assert ! (i < first_pos , "we should always arrive at first_pos: near index {:?}" , iter . get_current_position ()) ; i += 1 ; iter . advance_iter () ; if iter . is_eof () { iter . result_cache . clear () ; return Some (iter . len) ; } } } }
};
}
