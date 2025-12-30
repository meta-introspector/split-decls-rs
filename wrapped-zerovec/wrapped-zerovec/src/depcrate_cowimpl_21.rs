// Generated macro for impl_21 (impl)
macro_rules! Depcrate_cowimpl_21 {
() => {
// Module: crate::cow
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , V : VarULE + ? Sized > Deref for VarZeroCow < 'a , V > { type Target = V ; fn deref (& self) -> & V { unsafe { V :: from_bytes_unchecked (self . as_bytes ()) } } }
};
}
