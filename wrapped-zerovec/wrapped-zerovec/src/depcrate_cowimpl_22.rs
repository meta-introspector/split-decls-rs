// Generated macro for impl_22 (impl)
macro_rules! Depcrate_cowimpl_22 {
() => {
// Module: crate::cow
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a , V : VarULE + ? Sized > From < & 'a V > for VarZeroCow < 'a , V > { fn from (other : & 'a V) -> Self { Self :: new_borrowed (other) } }
};
}
