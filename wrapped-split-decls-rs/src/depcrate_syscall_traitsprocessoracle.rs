// Generated macro for ProcessOracle (trait)
macro_rules! Depcrate_syscall_traitsProcessOracle {
() => {
// Module: crate::syscall_traits
// Provides: {"ProcessOracle"}
// Dependencies: {}
# [doc = " Process execution oracle  "] pub trait ProcessOracle { fn audit_exec () -> Result < () , String > ; fn check_command_safety (cmd : & str) -> bool ; }
};
}
