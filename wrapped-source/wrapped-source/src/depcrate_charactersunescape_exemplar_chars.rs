// Generated macro for unescape_exemplar_chars (function)
macro_rules! Depcrate_charactersunescape_exemplar_chars {
() => {
// Module: crate::characters
// Provides: {"unescape_exemplar_chars"}
// Dependencies: {}
# [doc = " Unescape a (sub-)string of exemplar character data"] fn unescape_exemplar_chars (char_block : & str) -> String { if char_block . chars () . all (| ch | ch == '\\') { return "\\" . to_string () ; } else if char_block . chars () . all (| ch | ch == '\"' || ch == '＂' || ch == '\\') { return char_block . replace ('\\' , "") ; } let mut ch_vec = char_block . chars () . collect :: < Vec < char > > () ; let mut ch_indices_to_remove : Vec < usize > = vec ! [] ; for (idx , ch) in ch_vec . iter () . enumerate () . rev () { if ch == & '\\' { let ch_after_slash = ch_vec . get (idx + 1) . unwrap () ; if ch_after_slash != & 'u' && ch_after_slash != & 'U' { ch_indices_to_remove . push (idx) ; } } } for idx in ch_indices_to_remove { ch_vec . remove (idx) ; } let ch_for_toml = ch_vec . iter () . collect :: < String > () ; let mut ch_for_toml = ch_for_toml . to_string () ; for _i in 1 ..= 3 { ch_for_toml = ch_for_toml . replace ("\\\"" , "\"") ; } ch_for_toml = ch_for_toml . replace ('\"' , "\\\"") ; let ch_for_toml = format ! ("x=\"{ch_for_toml}\"") ; let ch_lite_t_val : toml :: Value = toml :: from_str (& ch_for_toml) . unwrap_or_else (| _ | panic ! ("{char_block:?}")) ; let ch_lite = if let toml :: Value :: Table (t) = ch_lite_t_val { if let Some (toml :: Value :: String (s)) = t . get ("x") { s . to_owned () } else { panic ! () ; } } else { panic ! () ; } ; let result = ch_lite . trim () . to_string () ; result }
};
}
