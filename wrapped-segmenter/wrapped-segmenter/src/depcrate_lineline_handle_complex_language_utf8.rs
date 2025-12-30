// Generated macro for line_handle_complex_language_utf8 (function)
macro_rules! Depcrate_lineline_handle_complex_language_utf8 {
() => {
// Module: crate::line
// Provides: {"line_handle_complex_language_utf8"}
// Dependencies: {}
# [doc = " line_handle_complex_language impl for UTF8 iterators"] fn line_handle_complex_language_utf8 < T > (iter : & mut LineBreakIterator < '_ , '_ , T > , left_codepoint : char ,) -> Option < usize > where T : LineBreakType < CharType = char > , { let start_iter = iter . iter . clone () ; let start_point = iter . current_pos_data ; let mut s = String :: new () ; s . push (left_codepoint) ; loop { debug_assert ! (! iter . is_eof ()) ; s . push (iter . get_current_codepoint () ?) ; iter . advance_iter () ; if let Some (current_codepoint) = iter . get_current_codepoint () { if ! T :: use_complex_breaking (iter , current_codepoint) { break ; } } else { break ; } } iter . iter = start_iter ; iter . current_pos_data = start_point ; let breaks = iter . complex . complex_language_segment_str (& s) ; iter . result_cache = breaks ; let first_pos = * iter . result_cache . first () ? ; let mut i = left_codepoint . len_utf8 () ; loop { if i == first_pos { iter . result_cache = iter . result_cache . iter () . skip (1) . map (| r | r - i) . collect () ; return iter . get_current_position () ; } debug_assert ! (i < first_pos , "we should always arrive at first_pos: near index {:?}" , iter . get_current_position ()) ; i += iter . get_current_codepoint () . map_or (0 , T :: char_len) ; iter . advance_iter () ; if iter . is_eof () { iter . result_cache . clear () ; return Some (iter . len) ; } } }
};
}
