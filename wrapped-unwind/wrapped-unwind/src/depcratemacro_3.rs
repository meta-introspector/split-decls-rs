// Generated macro for macro_3 (macro)
macro_rules! Depcratemacro_3 {
() => {
// Module: crate
// Provides: {"macro_3"}
// Dependencies: {}
# [cfg (target_env = "musl")] cfg_select ! { all (feature = "llvm-libunwind" , feature = "system-llvm-libunwind") => { compile_error ! ("`llvm-libunwind` and `system-llvm-libunwind` cannot be enabled at the same time") ; } feature = "llvm-libunwind" => { # [link (name = "unwind" , kind = "static" , modifiers = "-bundle")] unsafe extern "C" { } } feature = "system-llvm-libunwind" => { # [link (name = "unwind" , kind = "static" , modifiers = "-bundle" , cfg (target_feature = "crt-static"))] # [link (name = "unwind" , cfg (not (target_feature = "crt-static")))] unsafe extern "C" { } } _ => { # [link (name = "unwind" , kind = "static" , modifiers = "-bundle" , cfg (target_feature = "crt-static"))] # [link (name = "gcc_s" , cfg (not (target_feature = "crt-static")))] unsafe extern "C" { } } }
};
}
