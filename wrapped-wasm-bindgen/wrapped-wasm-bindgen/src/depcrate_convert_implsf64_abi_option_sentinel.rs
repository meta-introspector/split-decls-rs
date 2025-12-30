// Generated macro for F64_ABI_OPTION_SENTINEL (const)
macro_rules! Depcrate_convert_implsF64_ABI_OPTION_SENTINEL {
() => {
// Module: crate::convert::impls
// Provides: {"F64_ABI_OPTION_SENTINEL"}
// Dependencies: {}
# [doc = " The sentinel value is 2^32 + 1 for 32-bit primitive types."] # [doc = ""] # [doc = " 2^32 + 1 is used, because it's the smallest positive integer that cannot be"] # [doc = " represented by any 32-bit primitive. While any value >= 2^32 works as a"] # [doc = " sentinel value for 32-bit integers, it's a bit more tricky for `f32`. `f32`"] # [doc = " can represent all powers of 2 up to 2^127 exactly. And between 2^32 and 2^33,"] # [doc = " `f32` can represent all integers 2^32+512*k exactly."] const F64_ABI_OPTION_SENTINEL : f64 = 4294967297_f64 ;
};
}
