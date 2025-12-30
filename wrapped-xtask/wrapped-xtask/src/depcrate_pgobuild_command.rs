// Generated macro for build_command (function)
macro_rules! Depcrate_pgobuild_command {
() => {
// Module: crate::pgo
// Provides: {"build_command"}
// Dependencies: {}
# [doc = " Helper function to create a build command for rust-analyzer"] pub (crate) fn build_command < 'a > (sh : & 'a Shell , command : & str , target_name : & str , features : & [& str] ,) -> Cmd < 'a > { cmd ! (sh , "cargo {command} --manifest-path ./crates/rust-analyzer/Cargo.toml --bin rust-analyzer --target {target_name} {features...} --release") }
};
}
