// Generated macro for CAN_DIST_DYLIBS (const)
macro_rules! Depcrate_compiler_compilerCAN_DIST_DYLIBS {
() => {
// Module: crate::compiler::compiler
// Provides: {"CAN_DIST_DYLIBS"}
// Dependencies: {}
# [cfg (all (feature = "dist-client" , not (any (all (target_os = "linux" , target_arch = "x86_64") , target_os = "freebsd"))))] pub const CAN_DIST_DYLIBS : bool = false ;
};
}
