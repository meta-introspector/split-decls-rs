// Generated macro for impl_97 (impl)
macro_rules! Depcrate_data_runtimeimpl_97 {
() => {
// Module: crate::data::runtime
// Provides: {"impl_97"}
// Dependencies: {}
impl Span { fn from_pos (pos : & Position , file : & str) -> Span { let mut target_line = None ; let mut line_start = 0 ; for (i , line) in crate :: utils :: LinesWithTerminator :: new (file) . enumerate () { if i == pos . line as usize - 1 { # [allow (clippy :: skip_while_next)] let byte_offset = line . char_indices () . skip ((pos . column - 1) . try_into () . unwrap ()) . skip_while (| & (_ , c) | c != '!') . skip (1) . skip_while (| & (_ , c) | c . is_whitespace ()) . skip (1) . skip_while (| & (_ , c) | c . is_whitespace ()) . next () . expect ("Failed to parse macro invocation") . 0 ; let literal_start = line_start + byte_offset ; target_line = Some (literal_start) ; break ; } line_start += line . len () ; } let literal_start = target_line . unwrap () ; let lit_to_eof = & file [literal_start ..] ; let lit_to_eof_trimmed = lit_to_eof . trim_start () ; let literal_start = literal_start + (lit_to_eof . len () - lit_to_eof_trimmed . len ()) ; let literal_len = locate_end (lit_to_eof_trimmed) . expect ("Couldn't find closing delimiter for `expect!`.") ; let literal_range = literal_start .. literal_start + literal_len ; Span { literal_range } } }
};
}
