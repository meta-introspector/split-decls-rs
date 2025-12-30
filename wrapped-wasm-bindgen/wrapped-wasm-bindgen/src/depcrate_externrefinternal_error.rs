// Generated macro for internal_error (function)
macro_rules! Depcrate_externrefinternal_error {
() => {
// Module: crate::externref
// Provides: {"internal_error"}
// Dependencies: {}
fn internal_error (_msg : & str) -> ! { cfg_if :: cfg_if ! { if # [cfg (debug_assertions)] { super :: throw_str (_msg) } else if # [cfg (feature = "std")] { std :: process :: abort () ; } else if # [cfg (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none")))] { core :: arch :: wasm32 :: unreachable () ; } else { unreachable ! () } } }
};
}
