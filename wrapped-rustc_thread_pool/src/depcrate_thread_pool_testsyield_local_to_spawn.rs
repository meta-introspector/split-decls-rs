// Generated macro for yield_local_to_spawn (function)
macro_rules! Depcrate_thread_pool_testsyield_local_to_spawn {
() => {
// Module: crate::thread_pool::tests
// Provides: {"yield_local_to_spawn"}
// Dependencies: {}
# [test] fn yield_local_to_spawn () { let (tx , rx) = channel () ; crate :: spawn (move | | tx . send (22) . unwrap ()) ; crate :: registry :: in_worker (move | _ , _ | { crate :: yield_local () ; }) ; assert_eq ! (22 , rx . recv () . unwrap ()) ; }
};
}
