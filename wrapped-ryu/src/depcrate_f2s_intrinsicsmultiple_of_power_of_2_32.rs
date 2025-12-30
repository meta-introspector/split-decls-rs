// Generated macro for multiple_of_power_of_2_32 (function)
macro_rules! Depcrate_f2s_intrinsicsmultiple_of_power_of_2_32 {
() => {
// Module: crate::f2s_intrinsics
// Provides: {"multiple_of_power_of_2_32"}
// Dependencies: {}
# [cfg_attr (feature = "no-panic" , inline)] pub fn multiple_of_power_of_2_32 (value : u32 , p : u32) -> bool { (value & ((1u32 << p) - 1)) == 0 }
};
}
