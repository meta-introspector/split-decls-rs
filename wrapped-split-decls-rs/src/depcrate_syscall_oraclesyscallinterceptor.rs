// Generated macro for SyscallInterceptor (struct)
macro_rules! Depcrate_syscall_oracleSyscallInterceptor {
() => {
// Module: crate::syscall_oracle
// Provides: {"SyscallInterceptor"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct SyscallInterceptor { pub syscall_mappings : HashMap < String , SyscallWrapper > , pub mock_mode : bool , pub dao_governance : bool , pub type_safety_level : TypeSafetyLevel , }
};
}
