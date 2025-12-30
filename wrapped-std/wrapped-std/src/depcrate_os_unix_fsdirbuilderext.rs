// Generated macro for DirBuilderExt (trait)
macro_rules! Depcrate_os_unix_fsDirBuilderExt {
() => {
// Module: crate::os::unix::fs
// Provides: {"DirBuilderExt"}
// Dependencies: {}
# [doc = " Unix-specific extensions to [`fs::DirBuilder`]."] # [stable (feature = "dir_builder" , since = "1.6.0")] pub trait DirBuilderExt { # [doc = " Sets the mode to create new directories with. This option defaults to"] # [doc = " 0o777."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs::DirBuilder;"] # [doc = " use std::os::unix::fs::DirBuilderExt;"] # [doc = ""] # [doc = " let mut builder = DirBuilder::new();"] # [doc = " builder.mode(0o755);"] # [doc = " ```"] # [stable (feature = "dir_builder" , since = "1.6.0")] fn mode (& mut self , mode : u32) -> & mut Self ; }
};
}
