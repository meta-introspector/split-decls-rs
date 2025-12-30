// Generated macro for get_imp_inner (function)
macro_rules! Depcrate_imp_ssse3get_imp_inner {
() => {
// Module: crate::imp::ssse3
// Provides: {"get_imp_inner"}
// Dependencies: {}
# [inline] # [cfg (all (not (target_feature = "ssse3") , not (all (feature = "std" , any (target_arch = "x86" , target_arch = "x86_64")))))] fn get_imp_inner () -> Option < Adler32Imp > { None }
};
}
