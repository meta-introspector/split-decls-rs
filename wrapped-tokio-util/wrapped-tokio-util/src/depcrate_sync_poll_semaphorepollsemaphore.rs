// Generated macro for PollSemaphore (struct)
macro_rules! Depcrate_sync_poll_semaphorePollSemaphore {
() => {
// Module: crate::sync::poll_semaphore
// Provides: {"PollSemaphore"}
// Dependencies: {}
# [doc = " A wrapper around [`Semaphore`] that provides a `poll_acquire` method."] # [doc = ""] # [doc = " [`Semaphore`]: tokio::sync::Semaphore"] pub struct PollSemaphore { semaphore : Arc < Semaphore > , permit_fut : Option < (u32 , ReusableBoxFuture < 'static , Result < OwnedSemaphorePermit , AcquireError > > ,) > , }
};
}
