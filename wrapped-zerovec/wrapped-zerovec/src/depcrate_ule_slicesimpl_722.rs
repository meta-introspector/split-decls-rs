// Generated macro for impl_722 (impl)
macro_rules! Depcrate_ule_slicesimpl_722 {
() => {
// Module: crate::ule::slices
// Provides: {"impl_722"}
// Dependencies: {}
# [doc = " Note: VarULE is well-defined for all `[T]` where `T: ULE`, but [`ZeroSlice`] is more ergonomic"] # [doc = " when `T` is a low-level ULE type. For example:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use zerovec::ZeroSlice;"] # [doc = " # use zerovec::VarZeroVec;"] # [doc = " # use zerovec::ule::AsULE;"] # [doc = " // OK: [u8] is a useful type"] # [doc = " let _: VarZeroVec<[u8]> = unimplemented!();"] # [doc = ""] # [doc = " // Technically works, but [u32::ULE] is not very useful"] # [doc = " let _: VarZeroVec<[<u32 as AsULE>::ULE]> = unimplemented!();"] # [doc = ""] # [doc = " // Better: ZeroSlice<u32>"] # [doc = " let _: VarZeroVec<ZeroSlice<u32>> = unimplemented!();"] # [doc = " ```"] # [doc = ""] # [doc = " [`ZeroSlice`]: crate::ZeroSlice"] unsafe impl < T > VarULE for [T] where T : ULE , { # [inline] fn validate_bytes (slice : & [u8]) -> Result < () , UleError > { T :: validate_bytes (slice) } # [inline] unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { T :: slice_from_bytes_unchecked (bytes) } }
};
}
