// Generated macro for impl_231 (impl)
macro_rules! Depcrate_syscall_traitsimpl_231 {
() => {
// Module: crate::syscall_traits
// Provides: {"impl_231"}
// Dependencies: {}
impl ProcessOracle for DefaultProcessOracle { fn audit_exec () -> Result < () , String > { eprintln ! ("PROC_AUDIT: Exec operation") ; Ok (()) } fn check_command_safety (cmd : & str) -> bool { ! cmd . contains ("rm -rf") && ! cmd . contains ("sudo") } }
};
}
