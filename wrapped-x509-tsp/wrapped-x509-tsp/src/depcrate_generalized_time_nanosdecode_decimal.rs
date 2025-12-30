// Generated macro for decode_decimal (function)
macro_rules! Depcrate_generalized_time_nanosdecode_decimal {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"decode_decimal"}
// Dependencies: {}
# [doc = " Decode 2-digit decimal value"] # [allow (clippy :: arithmetic_side_effects)] fn decode_decimal (tag : Tag , hi : u8 , lo : u8) -> Result < u8 > { if hi . is_ascii_digit () && lo . is_ascii_digit () { Ok ((hi - b'0') * 10 + (lo - b'0')) } else { Err (tag . value_error () . into ()) } }
};
}
