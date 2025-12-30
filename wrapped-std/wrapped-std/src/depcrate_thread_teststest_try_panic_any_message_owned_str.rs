// Generated macro for test_try_panic_any_message_owned_str (function)
macro_rules! Depcrate_thread_teststest_try_panic_any_message_owned_str {
() => {
// Module: crate::thread::tests
// Provides: {"test_try_panic_any_message_owned_str"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_try_panic_any_message_owned_str () { match thread :: spawn (move | | { panic_any ("owned string" . to_string ()) ; }) . join () { Err (e) => { type T = String ; assert ! (e . is ::< T > ()) ; assert_eq ! (* e . downcast ::< T > () . unwrap () , "owned string" . to_string ()) ; } Ok (()) => panic ! () , } }
};
}
