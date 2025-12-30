// Generated macro for impl_506 (impl)
macro_rules! Depcrate_threadimpl_506 {
() => {
// Module: crate::thread
// Provides: {"impl_506"}
// Dependencies: {}
impl ThreadId { # [doc = " Create a new [`ThreadId`]."] fn new () -> Self { # [doc = " Separate failed [`ThreadId`] to apply `#[cold]` to it."] # [cold] fn exhausted () -> ! { panic ! ("failed to generate unique thread ID: bitspace exhausted") } # [doc = " Global counter for [`ThreadId`]."] static COUNTER : AtomicU64 = AtomicU64 :: new (0) ; let mut last = COUNTER . load (Ordering :: Relaxed) ; loop { let Some (id) = last . checked_add (1) else { exhausted () ; } ; match COUNTER . compare_exchange_weak (last , id , Ordering :: Relaxed , Ordering :: Relaxed) { Ok (_) => return Self (NonZeroU64 :: new (id) . expect ("unexpected `0` `ThreadId`")) , Err (id) => last = id , } } } }
};
}
