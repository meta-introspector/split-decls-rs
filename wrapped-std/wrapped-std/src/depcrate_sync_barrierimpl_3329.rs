// Generated macro for impl_3329 (impl)
macro_rules! Depcrate_sync_barrierimpl_3329 {
() => {
// Module: crate::sync::barrier
// Provides: {"impl_3329"}
// Dependencies: {}
impl BarrierWaitResult { # [doc = " Returns `true` if this thread is the \"leader thread\" for the call to"] # [doc = " [`Barrier::wait()`]."] # [doc = ""] # [doc = " Only one thread will have `true` returned from their result, all other"] # [doc = " threads will have `false` returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Barrier;"] # [doc = ""] # [doc = " let barrier = Barrier::new(1);"] # [doc = " let barrier_wait_result = barrier.wait();"] # [doc = " println!(\"{:?}\", barrier_wait_result.is_leader());"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [must_use] pub fn is_leader (& self) -> bool { self . 0 } }
};
}
