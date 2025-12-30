// Generated macro for impl_173 (impl)
macro_rules! Depcrate_sharedimpl_173 {
() => {
// Module: crate::shared
// Provides: {"impl_173"}
// Dependencies: {}
impl < T > Drop for Shared < T > { # [inline] fn drop (& mut self) { if unsafe { (* self . instance_ptr . as_ptr ()) . drop_ref () } { RefCounted :: pass_to_collector (self . instance_ptr . as_ptr ()) ; } } }
};
}
