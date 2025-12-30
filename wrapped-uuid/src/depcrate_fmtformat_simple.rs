// Generated macro for format_simple (function)
macro_rules! Depcrate_fmtformat_simple {
() => {
// Module: crate::fmt
// Provides: {"format_simple"}
// Dependencies: {}
# [inline] const fn format_simple (src : & [u8 ; 16] , upper : bool) -> [u8 ; 32] { let lut = if upper { & UPPER } else { & LOWER } ; let mut dst = [0 ; 32] ; let mut i = 0 ; while i < 16 { let x = src [i] ; dst [i * 2] = lut [(x >> 4) as usize] ; dst [i * 2 + 1] = lut [(x & 0x0f) as usize] ; i += 1 ; } dst }
};
}
