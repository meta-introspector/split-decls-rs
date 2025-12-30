// Generated macro for mul_div_u64 (function)
macro_rules! Depcrate_sys_commonmul_div_u64 {
() => {
// Module: crate::sys_common
// Provides: {"mul_div_u64"}
// Dependencies: {}
# [allow (dead_code)] pub fn mul_div_u64 (value : u64 , numerator : u64 , denom : u64) -> u64 { let q = value / denom ; let r = value % denom ; q * numerator + r * numerator / denom }
};
}
