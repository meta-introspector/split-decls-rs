// Generated macro for impl_170 (impl)
macro_rules! Depcrate_sharedimpl_170 {
() => {
// Module: crate::shared
// Provides: {"impl_170"}
// Dependencies: {}
impl < T > AsRef < T > for Shared < T > { # [inline] fn as_ref (& self) -> & T { unsafe { & * self . instance_ptr . as_ptr () } } }
};
}
