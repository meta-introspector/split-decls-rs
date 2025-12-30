// Generated macro for racy_clear (function)
macro_rules! Depcrate_tests_loom_poolracy_clear {
() => {
// Module: crate::tests::loom_pool
// Provides: {"racy_clear"}
// Dependencies: {}
# [test] fn racy_clear () { run_model ("racy_clear" , | | { let pool = Arc :: new (Pool :: new ()) ; let (item , value) = DontDropMe :: new (1) ; let idx = pool . create_with (move | item | * item = value) . expect ("create_with") ; assert_eq ! (pool . get (idx) . unwrap () . 0 . id , item . id) ; let p = pool . clone () ; let t2 = thread :: spawn (move | | p . clear (idx)) ; let r1 = pool . clear (idx) ; let r2 = t2 . join () . expect ("thread 2 should not panic") ; test_println ! ("r1: {}, r2: {}" , r1 , r2) ; assert ! (! (r1 && r2) , "Both threads should not have cleared the value") ; assert ! (r1 || r2 , "One thread should have removed the value") ; assert ! (pool . get (idx) . is_none ()) ; item . assert_clear () ; }) }
};
}
