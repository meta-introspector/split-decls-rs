// Generated macro for scope_spawn_broadcast_barrier (function)
macro_rules! Depcrate_scope_testsscope_spawn_broadcast_barrier {
() => {
// Module: crate::scope::tests
// Provides: {"scope_spawn_broadcast_barrier"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn scope_spawn_broadcast_barrier () { let barrier = Barrier :: new (8) ; let pool = ThreadPoolBuilder :: new () . num_threads (7) . build () . unwrap () ; pool . in_place_scope (| s | { s . spawn_broadcast (| _ , _ | { barrier . wait () ; }) ; barrier . wait () ; }) ; }
};
}
