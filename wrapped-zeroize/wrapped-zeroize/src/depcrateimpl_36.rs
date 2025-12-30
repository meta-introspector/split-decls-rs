// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
# [doc = " Impl [`Zeroize`] on [`MaybeUninit`] types."] # [doc = ""] # [doc = " This fills the memory with zeroes."] # [doc = " Note that this ignore invariants that `Z` might have, because"] # [doc = " [`MaybeUninit`] removes all invariants."] impl < Z > Zeroize for MaybeUninit < Z > { fn zeroize (& mut self) { unsafe { ptr :: write_volatile (self , MaybeUninit :: zeroed ()) } atomic_fence () ; } }
};
}
