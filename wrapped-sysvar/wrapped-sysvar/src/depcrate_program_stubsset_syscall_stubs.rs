// Generated macro for set_syscall_stubs (function)
macro_rules! Depcrate_program_stubsset_syscall_stubs {
() => {
// Module: crate::program_stubs
// Provides: {"set_syscall_stubs"}
// Dependencies: {}
pub fn set_syscall_stubs (syscall_stubs : Box < dyn SyscallStubs >) -> Box < dyn SyscallStubs > { std :: mem :: replace (& mut SYSCALL_STUBS . write () . unwrap () , syscall_stubs) }
};
}
