// Generated macro for get_imp_inner (function)
macro_rules! Depcrate_imp_wasmget_imp_inner {
() => {
// Module: crate::imp::wasm
// Provides: {"get_imp_inner"}
// Dependencies: {}
# [inline] # [cfg (not (all (target_feature = "simd128" , any (target_arch = "wasm32" , all (feature = "nightly" , target_arch = "wasm64")))))] fn get_imp_inner () -> Option < Adler32Imp > { None }
};
}
