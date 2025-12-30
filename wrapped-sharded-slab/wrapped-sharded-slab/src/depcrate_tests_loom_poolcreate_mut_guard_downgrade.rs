// Generated macro for create_mut_guard_downgrade (function)
macro_rules! Depcrate_tests_loom_poolcreate_mut_guard_downgrade {
() => {
// Module: crate::tests::loom_pool
// Provides: {"create_mut_guard_downgrade"}
// Dependencies: {}
# [test] fn create_mut_guard_downgrade () { run_model ("create_mut_guard_downgrade" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . create () . unwrap () ; let key : usize = guard . key () ; let pool2 = pool . clone () ; let pool3 = pool . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (pool2 . get (key)) ; }) ; guard . push_str ("Hello world") ; let guard = guard . downgrade () ; let t2 = thread :: spawn (move | | { test_dbg ! (pool3 . get (key)) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; assert_eq ! (guard , "Hello world" . to_owned ()) ; }) ; }
};
}
