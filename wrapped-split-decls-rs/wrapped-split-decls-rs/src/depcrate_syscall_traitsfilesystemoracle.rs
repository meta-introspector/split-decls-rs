// Generated macro for FileSystemOracle (trait)
macro_rules! Depcrate_syscall_traitsFileSystemOracle {
() => {
// Module: crate::syscall_traits
// Provides: {"FileSystemOracle"}
// Dependencies: {}
# [doc = " Filesystem operations oracle"] pub trait FileSystemOracle { fn audit_read () -> Result < () , String > ; fn audit_write () -> Result < () , String > ; fn check_path_safety (path : & str) -> bool ; }
};
}
