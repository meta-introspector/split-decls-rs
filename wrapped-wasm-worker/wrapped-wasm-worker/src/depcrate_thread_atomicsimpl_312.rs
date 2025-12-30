// Generated macro for impl_312 (impl)
macro_rules! Depcrate_thread_atomicsimpl_312 {
() => {
// Module: crate::thread::atomics
// Provides: {"impl_312"}
// Dependencies: {}
impl Scope { # [doc = " Creates a new [`Scope`]."] pub (super) fn new () -> Self { Self (Arc :: new (ScopeData { threads : AtomicU64 :: new (0) , thread : super :: current () , waker : AtomicWaker :: new () , })) } # [doc = " Returns the number of current threads."] pub (super) fn thread_count (& self) -> u64 { self . 0 . threads . load (Ordering :: Relaxed) } # [doc = " End the scope after calling the supplied function."] pub (super) fn finish (& self) { while self . 0 . threads . load (Ordering :: Acquire) != 0 { super :: park () ; } } # [doc = " End the scope after calling the supplied function."] pub (super) fn finish_async (& self , cx : & Context < '_ >) -> Poll < () > { if self . 0 . threads . load (Ordering :: Acquire) == 0 { return Poll :: Ready (()) ; } self . 0 . waker . register (cx . waker ()) ; if self . 0 . threads . load (Ordering :: Acquire) == 0 { Poll :: Ready (()) } else { Poll :: Pending } } }
};
}
