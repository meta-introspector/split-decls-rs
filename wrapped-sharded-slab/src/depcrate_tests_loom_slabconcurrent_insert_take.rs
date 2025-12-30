// Generated macro for concurrent_insert_take (function)
macro_rules! Depcrate_tests_loom_slabconcurrent_insert_take {
() => {
// Module: crate::tests::loom_slab
// Provides: {"concurrent_insert_take"}
// Dependencies: {}
# [test] fn concurrent_insert_take () { run_model ("concurrent_insert_remove" , | | { let slab = Arc :: new (Slab :: new ()) ; let pair = Arc :: new ((Mutex :: new (None) , Condvar :: new ())) ; let slab2 = slab . clone () ; let pair2 = pair . clone () ; let remover = thread :: spawn (move | | { let (lock , cvar) = & * pair2 ; for i in 0 .. 2 { test_println ! ("--- remover i={} ---" , i) ; let mut next = lock . lock () . unwrap () ; while next . is_none () { next = cvar . wait (next) . unwrap () ; } let key = next . take () . unwrap () ; assert_eq ! (slab2 . take (key) , Some (i)) ; cvar . notify_one () ; } }) ; let (lock , cvar) = & * pair ; for i in 0 .. 2 { test_println ! ("--- inserter i={} ---" , i) ; let key = slab . insert (i) . expect ("insert") ; let mut next = lock . lock () . unwrap () ; * next = Some (key) ; cvar . notify_one () ; while next . is_some () { next = cvar . wait (next) . unwrap () ; } assert ! (slab . get (key) . is_none ()) ; } remover . join () . unwrap () ; }) }
};
}
