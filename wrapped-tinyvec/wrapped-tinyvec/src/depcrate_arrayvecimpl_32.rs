// Generated macro for impl_32 (impl)
macro_rules! Depcrate_arrayvecimpl_32 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_32"}
// Dependencies: {}
impl < A > ArrayVec < A > { # [doc = " Returns the reference to the inner array of the `ArrayVec`."] # [doc = ""] # [doc = " This returns the full array, even if the `ArrayVec` length is currently"] # [doc = " less than that."] # [inline (always)] # [must_use] pub const fn as_inner (& self) -> & A { & self . data } }
};
}
