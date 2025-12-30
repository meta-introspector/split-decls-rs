// Generated macro for impl_111 (impl)
macro_rules! Depcrate_sync_poll_semaphoreimpl_111 {
() => {
// Module: crate::sync::poll_semaphore
// Provides: {"impl_111"}
// Dependencies: {}
impl Stream for PollSemaphore { type Item = OwnedSemaphorePermit ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < OwnedSemaphorePermit > > { Pin :: into_inner (self) . poll_acquire (cx) } }
};
}
