// Generated macro for impl_73 (impl)
macro_rules! Depcrate_taskimpl_73 {
() => {
// Module: crate::task
// Provides: {"impl_73"}
// Dependencies: {}
impl < T : Future > Spawn < T > { # [doc = " If `T` is a [`Future`] then poll it. This will handle pinning and the context"] # [doc = " type for the future."] pub fn poll (& mut self) -> Poll < T :: Output > { let fut = self . future . as_mut () ; self . task . enter (| cx | fut . poll (cx)) } }
};
}
