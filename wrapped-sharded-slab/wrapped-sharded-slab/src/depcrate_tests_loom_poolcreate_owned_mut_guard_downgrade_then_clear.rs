// Generated macro for create_owned_mut_guard_downgrade_then_clear (function)
macro_rules! Depcrate_tests_loom_poolcreate_owned_mut_guard_downgrade_then_clear {
() => {
// Module: crate::tests::loom_pool
// Provides: {"create_owned_mut_guard_downgrade_then_clear"}
// Dependencies: {}
# [test] fn create_owned_mut_guard_downgrade_then_clear () { run_model ("create_owned_mut_guard_downgrade_then_clear" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . clone () . create_owned () . unwrap () ; let key : usize = guard . key () ; let pool2 = pool . clone () ; guard . push_str ("Hello world") ; let guard = guard . downgrade () ; let pool3 = pool . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (pool2 . get (key)) ; }) ; let t2 = thread :: spawn (move | | { test_dbg ! (pool3 . clear (key)) ; }) ; assert_eq ! (guard , "Hello world" . to_owned ()) ; drop (guard) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; assert ! (pool . get (key) . is_none ()) ; }) ; }
};
}
