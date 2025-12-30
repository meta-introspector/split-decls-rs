// Generated macro for impl_119 (impl)
macro_rules! Depcrate_ownedimpl_119 {
() => {
// Module: crate::owned
// Provides: {"impl_119"}
// Dependencies: {}
impl < T > AsRef < T > for Owned < T > { # [inline] fn as_ref (& self) -> & T { unsafe { & * self . instance_ptr . as_ptr () } } }
};
}
