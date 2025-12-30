// Generated macro for trim_end_but_line_feed (function)
macro_rules! Depcrate_stringtrim_end_but_line_feed {
() => {
// Module: crate::string
// Provides: {"trim_end_but_line_feed"}
// Dependencies: {}
# [doc = " Trims whitespaces to the right except for the line feed character."] fn trim_end_but_line_feed (trim_end : bool , result : String) -> String { let whitespace_except_line_feed = | c : char | c . is_whitespace () && c != '\n' ; if trim_end && result . ends_with (whitespace_except_line_feed) { result . trim_end_matches (whitespace_except_line_feed) . to_string () } else { result } }
};
}
