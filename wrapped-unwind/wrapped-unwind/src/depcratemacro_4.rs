// Generated macro for macro_4 (macro)
macro_rules! Depcratemacro_4 {
() => {
// Module: crate
// Provides: {"macro_4"}
// Dependencies: {}
# [cfg (target_env = "ohos")] cfg_select ! { all (feature = "llvm-libunwind" , feature = "system-llvm-libunwind") => { compile_error ! ("`llvm-libunwind` and `system-llvm-libunwind` cannot be enabled at the same time") ; } feature = "llvm-libunwind" => { # [link (name = "unwind" , kind = "static" , modifiers = "-bundle")] unsafe extern "C" { } } _ => { # [link (name = "unwind" , kind = "static" , modifiers = "-bundle" , cfg (target_feature = "crt-static"))] # [link (name = "unwind" , cfg (not (target_feature = "crt-static")))] unsafe extern "C" { } } }
};
}
