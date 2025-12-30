// Generated macro for concurrent_create_with_clear (function)
macro_rules! Depcrate_tests_loom_poolconcurrent_create_with_clear {
() => {
// Module: crate::tests::loom_pool
// Provides: {"concurrent_create_with_clear"}
// Dependencies: {}
# [test] fn concurrent_create_with_clear () { run_model ("concurrent_create_with_clear" , | | { let pool : Arc < Pool < DontDropMe > > = Arc :: new (Pool :: new ()) ; let pair = Arc :: new ((Mutex :: new (None) , Condvar :: new ())) ; let (item1 , value) = DontDropMe :: new (1) ; let idx1 = pool . create_with (move | item | * item = value) . expect ("create_with") ; let p = pool . clone () ; let pair2 = pair . clone () ; let test_value = item1 . clone () ; let t1 = thread :: spawn (move | | { let (lock , cvar) = & * pair2 ; test_println ! ("-> making get request") ; assert_eq ! (p . get (idx1) . unwrap () . 0 . id , test_value . id) ; let mut next = lock . lock () . unwrap () ; * next = Some (()) ; cvar . notify_one () ; }) ; test_println ! ("-> making get request") ; let guard = pool . get (idx1) ; let (lock , cvar) = & * pair ; let mut next = lock . lock () . unwrap () ; while next . is_none () { next = cvar . wait (next) . unwrap () ; } assert ! (pool . clear (idx1)) ; item1 . assert_not_clear () ; t1 . join () . expect ("thread 1 unable to join") ; drop (guard) ; item1 . assert_clear () ; }) }
};
}
