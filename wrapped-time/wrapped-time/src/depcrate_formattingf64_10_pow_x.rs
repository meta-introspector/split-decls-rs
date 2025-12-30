// Generated macro for f64_10_pow_x (function)
macro_rules! Depcrate_formattingf64_10_pow_x {
() => {
// Module: crate::formatting
// Provides: {"f64_10_pow_x"}
// Dependencies: {}
# [doc = " Helper function to obtain 10^x, guaranteeing determinism for x ≤ 9. For these cases, the"] # [doc = " function optimizes to a lookup table. For x ≥ 10, it falls back to `10_f64.powi(x)`. The only"] # [doc = " situation where this would occur is if the user explicitly requests such precision when"] # [doc = " configuring the ISO 8601 well known format. All other possibilities max out at nine digits."] # [inline] fn f64_10_pow_x (x : NonZero < u8 >) -> f64 { match x . get () { 1 => 10. , 2 => 100. , 3 => 1_000. , 4 => 10_000. , 5 => 100_000. , 6 => 1_000_000. , 7 => 10_000_000. , 8 => 100_000_000. , 9 => 1_000_000_000. , x => 10_f64 . powi (x . cast_signed () . extend ()) , } }
};
}
