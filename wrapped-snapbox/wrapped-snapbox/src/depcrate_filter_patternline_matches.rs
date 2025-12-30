// Generated macro for line_matches (function)
macro_rules! Depcrate_filter_patternline_matches {
() => {
// Module: crate::filter::pattern
// Provides: {"line_matches"}
// Dependencies: {}
fn line_matches (mut actual : & str , expected : & str , redactions : & Redactions) -> bool { if actual == expected { return true ; } let expected = redactions . clear_unused (expected) ; let mut sections = expected . split ("[..]") . peekable () ; while let Some (section) = sections . next () { if let Some (remainder) = actual . strip_prefix (section) { if let Some (next_section) = sections . peek () { if next_section . is_empty () { actual = "" ; } else if let Some (restart_index) = remainder . find (next_section) { actual = & remainder [restart_index ..] ; } } else { return remainder . is_empty () ; } } else { return false ; } } false }
};
}
