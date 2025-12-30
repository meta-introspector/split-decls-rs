// Generated macro for skip_ansi_escape_sequence (function)
macro_rules! Depcrate_coreskip_ansi_escape_sequence {
() => {
// Module: crate::core
// Provides: {"skip_ansi_escape_sequence"}
// Dependencies: {}
# [doc = " Skip ANSI escape sequences."] # [doc = ""] # [doc = " The `ch` is the current `char`, the `chars` provide the following"] # [doc = " characters. The `chars` will be modified if `ch` is the start of"] # [doc = " an ANSI escape sequence."] # [doc = ""] # [doc = " Returns `true` if one or more chars were skipped."] # [inline] pub (crate) fn skip_ansi_escape_sequence < I : Iterator < Item = char > > (ch : char , chars : & mut I) -> bool { if ch != CSI . 0 { return false ; } let next = chars . next () ; if next == Some (CSI . 1) { for ch in chars { if ANSI_FINAL_BYTE . contains (& ch) { break ; } } } else if next == Some (']') { let mut last = ']' ; for new in chars { if new == '\x07' || (new == '\\' && last == CSI . 0) { break ; } last = new ; } } true }
};
}
