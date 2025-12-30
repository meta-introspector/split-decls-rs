// Generated macro for create_owned_mut_downgrade_during_clear (function)
macro_rules! Depcrate_tests_loom_poolcreate_owned_mut_downgrade_during_clear {
() => {
// Module: crate::tests::loom_pool
// Provides: {"create_owned_mut_downgrade_during_clear"}
// Dependencies: {}
# [test] fn create_owned_mut_downgrade_during_clear () { run_model ("create_owned_mut_downgrade_during_clear" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . clone () . create_owned () . unwrap () ; let key : usize = guard . key () ; guard . push_str ("Hello world") ; let pool2 = pool . clone () ; let guard = guard . downgrade () ; let t1 = thread :: spawn (move | | { test_dbg ! (pool2 . clear (key)) ; }) ; t1 . join () . unwrap () ; assert_eq ! (guard , "Hello world" . to_owned ()) ; drop (guard) ; assert ! (pool . get (key) . is_none ()) ; }) ; }
};
}
