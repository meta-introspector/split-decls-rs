// Generated macro for impl_72 (impl)
macro_rules! Depcrate_taskimpl_72 {
() => {
// Module: crate::task
// Provides: {"impl_72"}
// Dependencies: {}
impl < T : Unpin > ops :: DerefMut for Spawn < T > { fn deref_mut (& mut self) -> & mut T { & mut self . future } }
};
}
