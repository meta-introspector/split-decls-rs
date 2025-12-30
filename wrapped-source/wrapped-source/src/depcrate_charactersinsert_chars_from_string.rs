// Generated macro for insert_chars_from_string (function)
macro_rules! Depcrate_charactersinsert_chars_from_string {
() => {
// Module: crate::characters
// Provides: {"insert_chars_from_string"}
// Dependencies: {}
# [doc = " Parse the input string, and insert the represented exemplar \"characters\" (each of"] # [doc = " which could either be an individual code point or a code point sequence) into the set."] fn insert_chars_from_string (set : & mut HashSet < String > , input : & str) { let s = if input . chars () . count () > 1 && input . starts_with ('\\') { input . chars () . skip_while (| ch | ch == & '\\') . collect :: < String > () } else { input . to_string () } ; if s . contains ('-') && s . find ('-') . unwrap () > 0 { let (begin , end) = s . split_once ('-') . unwrap () ; let begin_char = begin . chars () . next_back () . unwrap () ; let end_char = end . chars () . next () . unwrap () ; for code_point in (begin_char as u32) ..= (end_char as u32) { let char_str = char :: from_u32 (code_point) . expect ("Character range should not span non-Unicode-scalar-value code points") . to_string () ; set . insert (char_str) ; } let rem_begin_str = & begin [.. (begin . len () - begin_char . len_utf8 ())] ; let rem_end_str = & end [end_char . len_utf8 () ..] ; insert_chars_from_string (set , rem_begin_str) ; insert_chars_from_string (set , rem_end_str) ; } else { for ch in s . chars () { set . insert (ch . to_string ()) ; } } }
};
}
