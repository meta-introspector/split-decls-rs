// Generated macro for handle_complex_language_utf8 (function)
macro_rules! Depcrate_wordhandle_complex_language_utf8 {
() => {
// Module: crate::word
// Provides: {"handle_complex_language_utf8"}
// Dependencies: {}
# [doc = " handle_complex_language impl for UTF8 iterators"] fn handle_complex_language_utf8 < T > (iter : & mut RuleBreakIterator < '_ , '_ , T > , left_codepoint : T :: CharType ,) -> Option < usize > where T : RuleBreakType < CharType = char > , { let start_iter = iter . iter . clone () ; let start_point = iter . current_pos_data ; let mut s = String :: new () ; s . push (left_codepoint) ; loop { debug_assert ! (! iter . is_eof ()) ; s . push (iter . get_current_codepoint () ?) ; iter . advance_iter () ; if let Some (current_break_property) = iter . get_current_break_property () { if current_break_property != iter . data . complex_property { break ; } } else { break ; } } iter . iter = start_iter ; iter . current_pos_data = start_point ; # [expect (clippy :: unwrap_used)] let breaks = iter . complex . unwrap () . complex_language_segment_str (& s) ; iter . result_cache = breaks ; let first_pos = * iter . result_cache . first () ? ; let mut i = left_codepoint . len_utf8 () ; loop { if i == first_pos { iter . result_cache = iter . result_cache . iter () . skip (1) . map (| r | r - i) . collect () ; return iter . get_current_position () ; } debug_assert ! (i < first_pos , "we should always arrive at first_pos: near index {:?}" , iter . get_current_position ()) ; i += iter . get_current_codepoint () . map_or (0 , T :: char_len) ; iter . advance_iter () ; if iter . is_eof () { iter . result_cache . clear () ; return Some (iter . len) ; } } }
};
}
