// Generated macro for impl_74 (impl)
macro_rules! Depcrate_taskimpl_74 {
() => {
// Module: crate::task
// Provides: {"impl_74"}
// Dependencies: {}
impl < T : Stream > Spawn < T > { # [doc = " If `T` is a [`Stream`] then `poll_next` it. This will handle pinning and the context"] # [doc = " type for the stream."] pub fn poll_next (& mut self) -> Poll < Option < T :: Item > > { let stream = self . future . as_mut () ; self . task . enter (| cx | stream . poll_next (cx)) } }
};
}
