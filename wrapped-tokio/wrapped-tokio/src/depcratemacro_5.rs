// Generated macro for macro_5 (macro)
macro_rules! Depcratemacro_5 {
() => {
// Module: crate
// Provides: {"macro_5"}
// Dependencies: {}
# [cfg (all (feature = "taskdump" , not (doc) , not (all (target_os = "linux" , any (target_arch = "aarch64" , target_arch = "x86" , target_arch = "x86_64")))))] compile_error ! ("The `taskdump` feature is only currently supported on \
linux, on `aarch64`, `x86` and `x86_64`.") ;
};
}
