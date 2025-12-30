// Generated macro for create_mut_guard_prevents_access (function)
macro_rules! Depcrate_tests_loom_poolcreate_mut_guard_prevents_access {
() => {
// Module: crate::tests::loom_pool
// Provides: {"create_mut_guard_prevents_access"}
// Dependencies: {}
# [test] fn create_mut_guard_prevents_access () { run_model ("create_mut_guard_prevents_access" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let guard = pool . create () . unwrap () ; let key : usize = guard . key () ; let pool2 = pool . clone () ; thread :: spawn (move | | { assert ! (pool2 . get (key) . is_none ()) ; }) . join () . unwrap () ; }) ; }
};
}
