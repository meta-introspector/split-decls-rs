// Generated macro for impl_51 (impl)
macro_rules! Depcrate_arrayvecimpl_51 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_51"}
// Dependencies: {}
impl < A : Array > ArrayVecIterator < A > { # [doc = " Returns the remaining items of this iterator as a slice."] # [inline] # [must_use] pub fn as_slice (& self) -> & [A :: Item] { & self . data . as_slice () [self . base as usize .. self . tail as usize] } }
};
}
