// Generated macro for other_3560 (other)
macro_rules! Depcrate_sys_env_constsother_3560 {
() => {
// Module: crate::sys::env_consts
// Provides: {"other_3560"}
// Dependencies: {}
macro cfg_unordered ($ (# [cfg ($ cfg : meta)] $ os : item) * # [else] $ fallback : item) { $ (# [cfg ($ cfg)] $ os) * # [cfg (not (any ($ ($ cfg) ,*)))] $ fallback }
};
}
