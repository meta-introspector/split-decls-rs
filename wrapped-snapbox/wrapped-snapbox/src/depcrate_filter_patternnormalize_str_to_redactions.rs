// Generated macro for normalize_str_to_redactions (function)
macro_rules! Depcrate_filter_patternnormalize_str_to_redactions {
() => {
// Module: crate::filter::pattern
// Provides: {"normalize_str_to_redactions"}
// Dependencies: {}
fn normalize_str_to_redactions (actual : & str , expected : & str , redactions : & Redactions) -> String { if actual == expected { return actual . to_owned () ; } let mut normalized : Vec < & str > = Vec :: new () ; let mut actual_index = 0 ; let actual_lines : Vec < _ > = crate :: utils :: LinesWithTerminator :: new (actual) . collect () ; let mut expected_lines = crate :: utils :: LinesWithTerminator :: new (expected) . peekable () ; while let Some (expected_line) = expected_lines . next () { if is_line_elide (expected_line) { let Some (next_expected_line) = expected_lines . peek () else { normalized . push (expected_line) ; actual_index = actual_lines . len () ; break ; } ; let Some (index_offset) = actual_lines [actual_index ..] . iter () . position (| next_actual_line | { line_matches (next_actual_line , next_expected_line , redactions) }) else { break ; } ; normalized . push (expected_line) ; actual_index += index_offset ; } else { let Some (actual_line) = actual_lines . get (actual_index) else { break ; } ; if line_matches (actual_line , expected_line , redactions) { actual_index += 1 ; normalized . push (expected_line) ; } else { actual_index += 1 ; normalized . push (actual_line) ; } } } normalized . extend (actual_lines [actual_index ..] . iter () . copied ()) ; normalized . join ("") }
};
}
