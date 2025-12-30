// Generated macro for impl_171 (impl)
macro_rules! Depcrate_sharedimpl_171 {
() => {
// Module: crate::shared
// Provides: {"impl_171"}
// Dependencies: {}
impl < T > Clone for Shared < T > { # [inline] fn clone (& self) -> Self { unsafe { (* self . instance_ptr . as_ptr ()) . add_ref () } Self { instance_ptr : self . instance_ptr , } } }
};
}
