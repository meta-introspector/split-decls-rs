// Generated macro for impl_266 (impl)
macro_rules! Depcrate_loom_std_barrierimpl_266 {
() => {
// Module: crate::loom::std::barrier
// Provides: {"impl_266"}
// Dependencies: {}
impl BarrierWaitResult { # [doc = " Returns `true` if this thread is the \"leader thread\" for the call to"] # [doc = " [`Barrier::wait()`]."] # [doc = ""] # [doc = " Only one thread will have `true` returned from their result, all other"] # [doc = " threads will have `false` returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Barrier;"] # [doc = ""] # [doc = " let barrier = Barrier::new(1);"] # [doc = " let barrier_wait_result = barrier.wait();"] # [doc = " println!(\"{:?}\", barrier_wait_result.is_leader());"] # [doc = " ```"] # [must_use] pub (crate) fn is_leader (& self) -> bool { self . 0 } }
};
}
