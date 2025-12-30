// Generated macro for decode_fractional_secs (function)
macro_rules! Depcrate_generalized_time_nanosdecode_fractional_secs {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"decode_fractional_secs"}
// Dependencies: {}
# [doc = " Decode up to 9 digits of fractional seconds, returns them as nanoseconds."] # [doc = ""] # [doc = " Assumes DER encoding rules, so no trailing zeroes."] fn decode_fractional_secs (tag : Tag , fract : & [u8]) -> Result < u32 > { if fract . is_empty () || fract . len () > 9 { return Err (tag . value_error () . into ()) ; } if fract . last () == Some (& b'0') { return Err (tag . value_error () . into ()) ; } if ! fract [0] . is_ascii_digit () { return Err (tag . value_error () . into ()) ; } let fract_str = core :: str :: from_utf8 (fract) . map_err (| _ | tag . value_error ()) ? ; let fract_num = u32 :: from_str (fract_str) . map_err (| _ | tag . value_error ()) ? ; let out = fract_num * 10_u32 . pow (9 - u32 :: try_from (fract . len ()) ?) ; Ok (out) }
};
}
