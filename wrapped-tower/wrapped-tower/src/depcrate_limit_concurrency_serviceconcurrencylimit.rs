// Generated macro for ConcurrencyLimit (struct)
macro_rules! Depcrate_limit_concurrency_serviceConcurrencyLimit {
() => {
// Module: crate::limit::concurrency::service
// Provides: {"ConcurrencyLimit"}
// Dependencies: {}
# [doc = " Enforces a limit on the concurrent number of requests the underlying"] # [doc = " service can handle."] # [derive (Debug)] pub struct ConcurrencyLimit < T > { inner : T , semaphore : PollSemaphore , # [doc = " The currently acquired semaphore permit, if there is sufficient"] # [doc = " concurrency to send a new request."] # [doc = ""] # [doc = " The permit is acquired in `poll_ready`, and taken in `call` when sending"] # [doc = " a new request."] permit : Option < OwnedSemaphorePermit > , }
};
}
