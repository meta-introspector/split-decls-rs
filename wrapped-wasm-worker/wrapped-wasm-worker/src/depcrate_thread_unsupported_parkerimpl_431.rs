// Generated macro for impl_431 (impl)
macro_rules! Depcrate_thread_unsupported_parkerimpl_431 {
() => {
// Module: crate::thread::unsupported::parker
// Provides: {"impl_431"}
// Dependencies: {}
impl Parker { # [doc = " Creates a new [`Parker`]."] # [allow (clippy :: missing_const_for_fn)] pub (in super :: super) fn new (_ : ThreadId) -> Self { Self (AtomicBool :: new (false)) } # [doc = " Parks the thread."] pub (in super :: super) fn park (self : Pin < & Self >) { if self . 0 . swap (false , Ordering :: Relaxed) { return ; } wait (None) ; unreachable ! ("thread should have never woken up") ; } # [doc = " Parks the thread with a timeout."] pub (in super :: super) fn park_timeout (self : Pin < & Self > , timeout : Duration) { if self . 0 . swap (false , Ordering :: Relaxed) { return ; } wait (Some (timeout)) ; } # [doc = " Unparks the thread."] pub (in super :: super) fn unpark (self : Pin < & Self >) { self . 0 . store (true , Ordering :: Relaxed) ; } }
};
}
