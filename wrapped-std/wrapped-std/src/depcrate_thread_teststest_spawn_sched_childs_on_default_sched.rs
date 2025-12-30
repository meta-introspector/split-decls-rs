// Generated macro for test_spawn_sched_childs_on_default_sched (function)
macro_rules! Depcrate_thread_teststest_spawn_sched_childs_on_default_sched {
() => {
// Module: crate::thread::tests
// Provides: {"test_spawn_sched_childs_on_default_sched"}
// Dependencies: {}
# [test] fn test_spawn_sched_childs_on_default_sched () { let (tx , rx) = channel () ; thread :: spawn (move | | { thread :: spawn (move | | { tx . send (()) . unwrap () ; }) ; }) ; rx . recv () . unwrap () ; }
};
}
