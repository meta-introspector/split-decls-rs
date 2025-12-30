// Generated macro for test_try_panic_message_string_literal (function)
macro_rules! Depcrate_thread_teststest_try_panic_message_string_literal {
() => {
// Module: crate::thread::tests
// Provides: {"test_try_panic_message_string_literal"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_try_panic_message_string_literal () { match thread :: spawn (move | | { panic ! ("static string") ; }) . join () { Err (e) => { type T = & 'static str ; assert ! (e . is ::< T > ()) ; assert_eq ! (* e . downcast ::< T > () . unwrap () , "static string") ; } Ok (()) => panic ! () , } }
};
}
