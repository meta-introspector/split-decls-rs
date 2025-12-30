// Generated macro for multiple_of_power_of_2 (function)
macro_rules! Depcrate_d2s_intrinsicsmultiple_of_power_of_2 {
() => {
// Module: crate::d2s_intrinsics
// Provides: {"multiple_of_power_of_2"}
// Dependencies: {}
# [cfg_attr (feature = "no-panic" , inline)] pub fn multiple_of_power_of_2 (value : u64 , p : u32) -> bool { debug_assert ! (value != 0) ; debug_assert ! (p < 64) ; (value & ((1u64 << p) - 1)) == 0 }
};
}
