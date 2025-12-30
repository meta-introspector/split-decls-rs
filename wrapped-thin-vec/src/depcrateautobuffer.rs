// Generated macro for AutoBuffer (struct)
macro_rules! DepcrateAutoBuffer {
() => {
// Module: crate
// Provides: {"AutoBuffer"}
// Dependencies: {}
# [cfg (feature = "gecko-ffi")] # [repr (C , align (8))] struct AutoBuffer < T , const N : usize > { header : Header , buffer : mem :: MaybeUninit < [T ; N] > , }
};
}
