// Generated macro for DigitCount (trait)
macro_rules! Depcrate_ext_digit_countDigitCount {
() => {
// Module: crate::ext::digit_count
// Provides: {"DigitCount"}
// Dependencies: {}
# [doc = " A trait that indicates the formatted width of the value can be determined."] # [doc = ""] # [doc = " Note that this should not be implemented for any signed integers. This forces the caller to"] # [doc = " write the sign if desired."] pub (crate) trait DigitCount { # [doc = " The number of digits in the stringified value."] fn num_digits (self) -> u8 ; }
};
}
