// Generated macro for scope_join_race (function)
macro_rules! Depcrate_thread_testsscope_join_race {
() => {
// Module: crate::thread::tests
// Provides: {"scope_join_race"}
// Dependencies: {}
# [test] # [cfg (miri)] fn scope_join_race () { for _ in 0 .. 100 { let a_bool = AtomicBool :: new (false) ; thread :: scope (| s | { for _ in 0 .. 5 { s . spawn (| | a_bool . load (Ordering :: Relaxed)) ; } for _ in 0 .. 5 { s . spawn (| | a_bool . load (Ordering :: Relaxed)) ; } }) ; } }
};
}
