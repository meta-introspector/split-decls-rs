// Generated macro for hide_leading_numbers (function)
macro_rules! Depcrate_normalizehide_leading_numbers {
() => {
// Module: crate::normalize
// Provides: {"hide_leading_numbers"}
// Dependencies: {}
fn hide_leading_numbers (line : & mut String) { let n = line . bytes () . take_while (| b : & u8 | * b == b' ' || b . is_ascii_digit ()) . count () ; for i in 0 .. n { line . replace_range (i .. i + 1 , " ") ; } }
};
}
