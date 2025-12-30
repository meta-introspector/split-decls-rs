// Generated macro for test_scoped_threads_drop_result_before_join (function)
macro_rules! Depcrate_thread_teststest_scoped_threads_drop_result_before_join {
() => {
// Module: crate::thread::tests
// Provides: {"test_scoped_threads_drop_result_before_join"}
// Dependencies: {}
# [test] fn test_scoped_threads_drop_result_before_join () { let actually_finished = & AtomicBool :: new (false) ; struct X < 'scope , 'env > (& 'scope Scope < 'scope , 'env > , & 'env AtomicBool) ; impl Drop for X < '_ , '_ > { fn drop (& mut self) { thread :: sleep (Duration :: from_millis (20)) ; let actually_finished = self . 1 ; self . 0 . spawn (move | | { thread :: sleep (Duration :: from_millis (20)) ; actually_finished . store (true , Ordering :: Relaxed) ; }) ; } } thread :: scope (| s | { s . spawn (move | | { thread :: sleep (Duration :: from_millis (20)) ; X (s , actually_finished) }) ; }) ; assert ! (actually_finished . load (Ordering :: Relaxed)) ; }
};
}
