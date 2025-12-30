// Generated macro for compute_inv_pow5 (function)
macro_rules! Depcrate_d2s_small_tablecompute_inv_pow5 {
() => {
// Module: crate::d2s_small_table
// Provides: {"compute_inv_pow5"}
// Dependencies: {}
# [cfg_attr (feature = "no-panic" , inline)] pub unsafe fn compute_inv_pow5 (i : u32) -> (u64 , u64) { let base = (i + DOUBLE_POW5_TABLE . len () as u32 - 1) / DOUBLE_POW5_TABLE . len () as u32 ; let base2 = base * DOUBLE_POW5_TABLE . len () as u32 ; let offset = base2 - i ; debug_assert ! (base < DOUBLE_POW5_INV_SPLIT2 . len () as u32) ; let mul = * DOUBLE_POW5_INV_SPLIT2 . get_unchecked (base as usize) ; if offset == 0 { return mul ; } debug_assert ! (offset < DOUBLE_POW5_TABLE . len () as u32) ; let m = * DOUBLE_POW5_TABLE . get_unchecked (offset as usize) ; let b0 = m as u128 * (mul . 0 - 1) as u128 ; let b2 = m as u128 * mul . 1 as u128 ; let delta = pow5bits (base2 as i32) - pow5bits (i as i32) ; debug_assert ! (base < POW5_INV_OFFSETS . len () as u32) ; let shifted_sum = ((b0 >> delta) + (b2 << (64 - delta))) + 1 + ((* POW5_INV_OFFSETS . get_unchecked ((i / 16) as usize) >> ((i % 16) << 1)) & 3) as u128 ; (shifted_sum as u64 , (shifted_sum >> 64) as u64) }
};
}
