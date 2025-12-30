// Generated macro for default_pool (function)
macro_rules! Depcrate_testsdefault_pool {
() => {
// Module: crate::tests
// Provides: {"default_pool"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn default_pool () { ThreadPoolBuilder :: default () . build () . unwrap () ; }
};
}
