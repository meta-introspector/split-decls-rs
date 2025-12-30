// Generated macro for strip_padded_character (function)
macro_rules! Depcrate_durationstrip_padded_character {
() => {
// Module: crate::duration
// Provides: {"strip_padded_character"}
// Dependencies: {}
# [doc = " Strips multiples of the given character from the start of the string."] # [doc = " Returns padding size and modifies `s` to point to the stripped string."] fn strip_padded_character (s : & mut & str , c : char) -> u8 { let mut padding = 0 ; while s . starts_with (c) { padding += 1 ; * s = & s [1 ..] ; } padding }
};
}
