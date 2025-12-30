// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
# [doc = " Impl [`Zeroize`] on slices of [`MaybeUninit`] types."] # [doc = ""] # [doc = " This impl can eventually be optimized using an memset intrinsic,"] # [doc = " such as [`core::intrinsics::volatile_set_memory`]."] # [doc = ""] # [doc = " This fills the slice with zeroes."] # [doc = ""] # [doc = " Note that this ignore invariants that `Z` might have, because"] # [doc = " [`MaybeUninit`] removes all invariants."] impl < Z > Zeroize for [MaybeUninit < Z >] { fn zeroize (& mut self) { let ptr = self . as_mut_ptr () . cast :: < MaybeUninit < u8 > > () ; let size = self . len () . checked_mul (size_of :: < Z > ()) . unwrap () ; assert ! (size <= isize :: MAX as usize) ; unsafe { volatile_set (ptr , MaybeUninit :: zeroed () , size) } atomic_fence () ; } }
};
}
