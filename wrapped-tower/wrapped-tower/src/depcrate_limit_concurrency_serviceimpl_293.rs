// Generated macro for impl_293 (impl)
macro_rules! Depcrate_limit_concurrency_serviceimpl_293 {
() => {
// Module: crate::limit::concurrency::service
// Provides: {"impl_293"}
// Dependencies: {}
impl < T > ConcurrencyLimit < T > { # [doc = " Create a new concurrency limiter."] pub fn new (inner : T , max : usize) -> Self { Self :: with_semaphore (inner , Arc :: new (Semaphore :: new (max))) } # [doc = " Create a new concurrency limiter with a provided shared semaphore"] pub fn with_semaphore (inner : T , semaphore : Arc < Semaphore >) -> Self { ConcurrencyLimit { inner , semaphore : PollSemaphore :: new (semaphore) , permit : None , } } # [doc = " Get a reference to the inner service"] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Get a mutable reference to the inner service"] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Consume `self`, returning the inner service"] pub fn into_inner (self) -> T { self . inner } }
};
}
