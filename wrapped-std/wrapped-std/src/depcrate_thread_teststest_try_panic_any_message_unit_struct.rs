// Generated macro for test_try_panic_any_message_unit_struct (function)
macro_rules! Depcrate_thread_teststest_try_panic_any_message_unit_struct {
() => {
// Module: crate::thread::tests
// Provides: {"test_try_panic_any_message_unit_struct"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_try_panic_any_message_unit_struct () { struct Juju ; match thread :: spawn (move | | panic_any (Juju)) . join () { Err (ref e) if e . is :: < Juju > () => { } Err (_) | Ok (()) => panic ! () , } }
};
}
