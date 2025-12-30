// Generated macro for test_spawn_sched (function)
macro_rules! Depcrate_thread_teststest_spawn_sched {
() => {
// Module: crate::thread::tests
// Provides: {"test_spawn_sched"}
// Dependencies: {}
# [test] fn test_spawn_sched () { let (tx , rx) = channel () ; fn f (i : i32 , tx : Sender < () >) { let tx = tx . clone () ; thread :: spawn (move | | { if i == 0 { tx . send (()) . unwrap () ; } else { f (i - 1 , tx) ; } }) ; } f (10 , tx) ; rx . recv () . unwrap () ; }
};
}
