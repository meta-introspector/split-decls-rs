// Generated macro for impl_71 (impl)
macro_rules! Depcrate_taskimpl_71 {
() => {
// Module: crate::task
// Provides: {"impl_71"}
// Dependencies: {}
impl < T : Unpin > ops :: Deref for Spawn < T > { type Target = T ; fn deref (& self) -> & T { & self . future } }
};
}
