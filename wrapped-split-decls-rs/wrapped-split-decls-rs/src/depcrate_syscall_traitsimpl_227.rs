// Generated macro for impl_227 (impl)
macro_rules! Depcrate_syscall_traitsimpl_227 {
() => {
// Module: crate::syscall_traits
// Provides: {"impl_227"}
// Dependencies: {}
impl SyscallOracle for DefaultSyscallOracle { fn audit_call (function_name : & str , syscall_type : & str) { eprintln ! ("AUDIT: {} called syscall type: {}" , function_name , syscall_type) ; } fn pre_call_hook (syscall_type : & str) { eprintln ! ("PRE_HOOK: {}" , syscall_type) ; } fn post_call_hook (syscall_type : & str , result : & dyn std :: fmt :: Debug) { eprintln ! ("POST_HOOK: {} -> {:?}" , syscall_type , result) ; } }
};
}
