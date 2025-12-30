// Generated macro for test_try_panic_any_message_any (function)
macro_rules! Depcrate_thread_teststest_try_panic_any_message_any {
() => {
// Module: crate::thread::tests
// Provides: {"test_try_panic_any_message_any"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_try_panic_any_message_any () { match thread :: spawn (move | | { panic_any (Box :: new (413u16) as Box < dyn Any + Send >) ; }) . join () { Err (e) => { type T = Box < dyn Any + Send > ; assert ! (e . is ::< T > ()) ; let any = e . downcast :: < T > () . unwrap () ; assert ! (any . is ::< u16 > ()) ; assert_eq ! (* any . downcast ::< u16 > () . unwrap () , 413) ; } Ok (()) => panic ! () , } }
};
}
