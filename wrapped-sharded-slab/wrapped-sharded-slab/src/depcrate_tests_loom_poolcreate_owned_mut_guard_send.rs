// Generated macro for create_owned_mut_guard_send (function)
macro_rules! Depcrate_tests_loom_poolcreate_owned_mut_guard_send {
() => {
// Module: crate::tests::loom_pool
// Provides: {"create_owned_mut_guard_send"}
// Dependencies: {}
# [test] fn create_owned_mut_guard_send () { run_model ("create_owned_mut_guard" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . clone () . create_owned () . unwrap () ; let key : usize = guard . key () ; let pool2 = pool . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (pool2 . get (key)) ; }) ; let t2 = thread :: spawn (move | | { guard . push_str ("Hello world") ; drop (guard) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; }) ; }
};
}
