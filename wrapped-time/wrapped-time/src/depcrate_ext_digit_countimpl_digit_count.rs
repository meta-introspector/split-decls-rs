// Generated macro for impl_digit_count (macro)
macro_rules! Depcrate_ext_digit_countimpl_digit_count {
() => {
// Module: crate::ext::digit_count
// Provides: {"impl_digit_count"}
// Dependencies: {}
# [doc = " A macro to generate implementations of `DigitCount` for unsigned integers."] macro_rules ! impl_digit_count { ($ ($ t : ty) ,* $ (,) ?) => { $ (impl DigitCount for $ t { # [inline] fn num_digits (self) -> u8 { match self . checked_ilog10 () { Some (n) => n . truncate ::< u8 > () + 1 , None => 1 , } } }) * } ; }
};
}
