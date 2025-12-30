// Generated macro for other_8 (other)
macro_rules! Depcrateother_8 {
() => {
// Module: crate
// Provides: {"other_8"}
// Dependencies: {}
# [cfg (all (target_os = "linux" , any (target_env = "gnu" , target_env = "uclibc") , not (feature = "llvm-libunwind") , feature = "system-llvm-libunwind"))] # [link (name = "unwind" , cfg (not (target_feature = "crt-static")))] unsafe extern "C" { }
};
}
