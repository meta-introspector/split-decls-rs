// Generated macro for get_imp_inner (function)
macro_rules! Depcrate_imp_avx512get_imp_inner {
() => {
// Module: crate::imp::avx512
// Provides: {"get_imp_inner"}
// Dependencies: {}
# [inline] # [cfg (all (not (all (feature = "nightly" , target_feature = "avx512f" , target_feature = "avx512bw")) , not (all (feature = "std" , feature = "nightly" , any (target_arch = "x86" , target_arch = "x86_64")))))] fn get_imp_inner () -> Option < Adler32Imp > { None }
};
}
