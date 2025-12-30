// Generated macro for log2_pow5 (function)
macro_rules! Depcrate_commonlog2_pow5 {
() => {
// Module: crate::common
// Provides: {"log2_pow5"}
// Dependencies: {}
# [cfg_attr (feature = "no-panic" , inline)] # [allow (dead_code)] pub fn log2_pow5 (e : i32) -> i32 { debug_assert ! (e >= 0) ; debug_assert ! (e <= 3528) ; ((e as u32 * 1217359) >> 19) as i32 }
};
}
