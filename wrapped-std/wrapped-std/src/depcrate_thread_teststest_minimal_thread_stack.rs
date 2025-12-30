// Generated macro for test_minimal_thread_stack (function)
macro_rules! Depcrate_thread_teststest_minimal_thread_stack {
() => {
// Module: crate::thread::tests
// Provides: {"test_minimal_thread_stack"}
// Dependencies: {}
# [cfg (windows)] # [test] fn test_minimal_thread_stack () { use crate :: sync :: atomic :: AtomicU8 ; static COUNT : AtomicU8 = AtomicU8 :: new (0) ; let builder = thread :: Builder :: new () . stack_size (1) ; let before = builder . spawn (| | COUNT . fetch_add (1 , Ordering :: Relaxed)) . unwrap () . join () . unwrap () ; assert_eq ! (before , 0) ; assert_eq ! (COUNT . load (Ordering :: Relaxed) , 1) ; }
};
}
