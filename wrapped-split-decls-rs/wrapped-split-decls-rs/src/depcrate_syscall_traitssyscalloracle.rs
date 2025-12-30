// Generated macro for SyscallOracle (trait)
macro_rules! Depcrate_syscall_traitsSyscallOracle {
() => {
// Module: crate::syscall_traits
// Provides: {"SyscallOracle"}
// Dependencies: {}
# [doc = " Core trait for syscall auditing and safety"] pub trait SyscallOracle { fn audit_call (function_name : & str , syscall_type : & str) ; fn pre_call_hook (syscall_type : & str) ; fn post_call_hook (syscall_type : & str , result : & dyn std :: fmt :: Debug) ; }
};
}
