// Generated macro for use_2022 (pub_use)
macro_rules! Depcrate_runtimeuse_2022 {
() => {
// Module: crate::runtime
// Provides: {"use_2022"}
// Dependencies: {}
# [doc = " `siginfo_t`"] # [doc = ""] # [doc = " This type is guaranteed to have the same layout as `libc::siginfo_t`."] # [doc = ""] # [doc = " If we want to expose this in public APIs, we should encapsulate the"] # [doc = " `linux_raw_sys` type."] pub use linux_raw_sys :: general :: siginfo_t as Siginfo ;
};
}
