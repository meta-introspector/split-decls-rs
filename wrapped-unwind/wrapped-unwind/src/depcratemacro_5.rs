// Generated macro for macro_5 (macro)
macro_rules! Depcratemacro_5 {
() => {
// Module: crate
// Provides: {"macro_5"}
// Dependencies: {}
# [cfg (target_os = "android")] cfg_select ! { feature = "llvm-libunwind" => { compile_error ! ("`llvm-libunwind` is not supported for Android targets") ; } _ => { # [link (name = "unwind" , kind = "static" , modifiers = "-bundle" , cfg (target_feature = "crt-static"))] # [link (name = "unwind" , cfg (not (target_feature = "crt-static")))] unsafe extern "C" { } } }
};
}
