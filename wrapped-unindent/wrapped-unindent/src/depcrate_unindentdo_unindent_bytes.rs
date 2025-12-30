// Generated macro for do_unindent_bytes (function)
macro_rules! Depcrate_unindentdo_unindent_bytes {
() => {
// Module: crate::unindent
// Provides: {"do_unindent_bytes"}
// Dependencies: {}
fn do_unindent_bytes (s : & [u8] , preserve_empty_first_line : bool) -> Vec < u8 > { let ignore_first_line = ! preserve_empty_first_line && (s . starts_with (b"\n") || s . starts_with (b"\r\n")) ; let spaces = s . lines () . skip (1) . filter_map (count_spaces) . min () . unwrap_or (0) ; let mut result = Vec :: with_capacity (s . len ()) ; for (i , line) in s . lines () . enumerate () { if i > 1 || (i == 1 && ! ignore_first_line) { result . push (b'\n') ; } if i == 0 { result . extend_from_slice (line) ; } else if line . len () > spaces { result . extend_from_slice (& line [spaces ..]) ; } } result }
};
}
