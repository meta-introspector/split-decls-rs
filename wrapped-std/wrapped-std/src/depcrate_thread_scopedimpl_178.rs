// Generated macro for impl_178 (impl)
macro_rules! Depcrate_thread_scopedimpl_178 {
() => {
// Module: crate::thread::scoped
// Provides: {"impl_178"}
// Dependencies: {}
impl ScopeData { pub (super) fn increment_num_running_threads (& self) { if self . num_running_threads . fetch_add (1 , Ordering :: Relaxed) > usize :: MAX / 2 { self . overflow () ; } } # [cold] fn overflow (& self) { self . decrement_num_running_threads (false) ; panic ! ("too many running threads in thread scope") ; } pub (super) fn decrement_num_running_threads (& self , panic : bool) { if panic { self . a_thread_panicked . store (true , Ordering :: Relaxed) ; } if self . num_running_threads . fetch_sub (1 , Ordering :: Release) == 1 { self . main_thread . unpark () ; } } }
};
}
