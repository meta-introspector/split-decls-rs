// Generated macro for impl_1809 (impl)
macro_rules! Depcrate_thread_idimpl_1809 {
() => {
// Module: crate::thread::id
// Provides: {"impl_1809"}
// Dependencies: {}
# [cfg (linux_kernel)] impl Cpuid { # [doc = " Converts a `RawCpuid` into a `Cpuid`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `raw` must be the value of a valid Linux CPU ID."] # [inline] pub const unsafe fn from_raw (raw : RawCpuid) -> Self { Self (raw) } # [doc = " Converts a `Cpuid` into a `RawCpuid`."] # [inline] pub const fn as_raw (self) -> RawCpuid { self . 0 } }
};
}
