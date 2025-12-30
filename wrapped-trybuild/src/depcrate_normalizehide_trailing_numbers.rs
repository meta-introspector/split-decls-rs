// Generated macro for hide_trailing_numbers (function)
macro_rules! Depcrate_normalizehide_trailing_numbers {
() => {
// Module: crate::normalize
// Provides: {"hide_trailing_numbers"}
// Dependencies: {}
fn hide_trailing_numbers (line : & mut String) { for _ in 0 .. 2 { let digits = line . bytes () . rev () . take_while (u8 :: is_ascii_digit) . count () ; if digits == 0 || ! line [.. line . len () - digits] . ends_with (':') { return ; } line . truncate (line . len () - digits - 1) ; } }
};
}
