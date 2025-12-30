// Generated macro for log10_pow2 (function)
macro_rules! Depcrate_commonlog10_pow2 {
() => {
// Module: crate::common
// Provides: {"log10_pow2"}
// Dependencies: {}
# [cfg_attr (feature = "no-panic" , inline)] pub fn log10_pow2 (e : i32) -> u32 { debug_assert ! (e >= 0) ; debug_assert ! (e <= 1650) ; (e as u32 * 78913) >> 18 }
};
}
