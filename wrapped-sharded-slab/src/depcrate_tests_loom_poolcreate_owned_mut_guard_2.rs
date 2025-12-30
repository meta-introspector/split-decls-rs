// Generated macro for create_owned_mut_guard_2 (function)
macro_rules! Depcrate_tests_loom_poolcreate_owned_mut_guard_2 {
() => {
// Module: crate::tests::loom_pool
// Provides: {"create_owned_mut_guard_2"}
// Dependencies: {}
# [test] fn create_owned_mut_guard_2 () { run_model ("create_owned_mut_guard_2" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . clone () . create_owned () . unwrap () ; let key : usize = guard . key () ; let pool2 = pool . clone () ; let pool3 = pool . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (pool2 . get (key)) ; }) ; guard . push_str ("Hello world") ; let t2 = thread :: spawn (move | | { test_dbg ! (pool3 . get (key)) ; }) ; drop (guard) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; }) ; }
};
}
