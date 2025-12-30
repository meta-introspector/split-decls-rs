// Generated macro for BarrierWaitResult (struct)
macro_rules! Depcrate_sync_barrierBarrierWaitResult {
() => {
// Module: crate::sync::barrier
// Provides: {"BarrierWaitResult"}
// Dependencies: {}
# [doc = " A `BarrierWaitResult` is returned by [`Barrier::wait()`] when all threads"] # [doc = " in the [`Barrier`] have rendezvoused."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Barrier;"] # [doc = ""] # [doc = " let barrier = Barrier::new(1);"] # [doc = " let barrier_wait_result = barrier.wait();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct BarrierWaitResult (bool) ;
};
}
