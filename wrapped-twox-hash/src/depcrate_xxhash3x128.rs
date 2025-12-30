// Generated macro for X128 (struct)
macro_rules! Depcrate_xxhash3X128 {
() => {
// Module: crate::xxhash3
// Provides: {"X128"}
// Dependencies: {}
# [doc = " THis exists just to easily map the XXH3 algorithm to Rust as the"] # [doc = " algorithm describes 128-bit results as a pair of high and low u64"] # [doc = " values."] # [derive (Copy , Clone)] pub (crate) struct X128 { pub low : u64 , pub high : u64 , }
};
}
