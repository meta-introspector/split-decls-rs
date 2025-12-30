// Generated macro for char_boundary (function)
macro_rules! Depcrate_errorchar_boundary {
() => {
// Module: crate::error
// Provides: {"char_boundary"}
// Dependencies: {}
fn char_boundary (input : & [u8] , offset : usize) -> core :: ops :: Range < usize > { let len = input . len () ; if offset == len { return offset .. offset ; } let start = (0 .. (offset + 1) . min (len)) . rev () . find (| i | { input . get (* i) . copied () . map (is_utf8_char_boundary) . unwrap_or (false) }) . unwrap_or (0) ; let end = (offset + 1 .. len) . find (| i | { input . get (* i) . copied () . map (is_utf8_char_boundary) . unwrap_or (false) }) . unwrap_or (len) ; start .. end }
};
}
