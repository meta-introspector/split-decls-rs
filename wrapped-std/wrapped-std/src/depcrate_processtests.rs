// Generated macro for tests (module)
macro_rules! Depcrate_processtests {
() => {
// Module: crate::process
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , not (any (target_os = "emscripten" , target_os = "wasi" , target_env = "sgx" , target_os = "xous" , target_os = "trusty" ,))))] mod tests ;
};
}
