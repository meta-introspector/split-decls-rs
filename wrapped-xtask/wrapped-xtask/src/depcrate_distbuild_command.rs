// Generated macro for build_command (function)
macro_rules! Depcrate_distbuild_command {
() => {
// Module: crate::dist
// Provides: {"build_command"}
// Dependencies: {}
fn build_command < 'a > (sh : & 'a Shell , command : & str , target_name : & str , features : & [& str] , dev_rel : bool ,) -> Cmd < 'a > { let profile = if dev_rel { "dev-rel" } else { "release" } ; cmd ! (sh , "cargo {command} --manifest-path ./crates/rust-analyzer/Cargo.toml --bin rust-analyzer --target {target_name} {features...} --profile {profile}") }
};
}
