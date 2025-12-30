// Generated macro for impl_183 (impl)
macro_rules! Depcrate_thread_scopedimpl_183 {
() => {
// Module: crate::thread::scoped
// Provides: {"impl_183"}
// Dependencies: {}
# [stable (feature = "scoped_threads" , since = "1.63.0")] impl fmt :: Debug for Scope < '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Scope") . field ("num_running_threads" , & self . data . num_running_threads . load (Ordering :: Relaxed)) . field ("a_thread_panicked" , & self . data . a_thread_panicked . load (Ordering :: Relaxed)) . field ("main_thread" , & self . data . main_thread) . finish_non_exhaustive () } }
};
}
