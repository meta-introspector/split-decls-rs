// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
# [doc = " Impl [`Zeroize`] on slices of types that can be zeroized with [`Default`]."] # [doc = ""] # [doc = " This impl can eventually be optimized using an memset intrinsic,"] # [doc = " such as [`core::intrinsics::volatile_set_memory`]. For that reason the"] # [doc = " blanket impl on slices is bounded by [`DefaultIsZeroes`]."] # [doc = ""] # [doc = " To zeroize a mut slice of `Z: Zeroize` which does not impl"] # [doc = " [`DefaultIsZeroes`], call `iter_mut().zeroize()`."] impl < Z > Zeroize for [Z] where Z : DefaultIsZeroes , { fn zeroize (& mut self) { assert ! (self . len () <= isize :: MAX as usize) ; unsafe { volatile_set (self . as_mut_ptr () , Z :: default () , self . len ()) } ; atomic_fence () ; } }
};
}
