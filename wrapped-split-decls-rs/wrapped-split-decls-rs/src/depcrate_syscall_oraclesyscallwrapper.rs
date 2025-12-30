// Generated macro for SyscallWrapper (struct)
macro_rules! Depcrate_syscall_oracleSyscallWrapper {
() => {
// Module: crate::syscall_oracle
// Provides: {"SyscallWrapper"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct SyscallWrapper { pub original_call : String , pub wrapper_macro : String , pub oracle_type : OracleType , pub safety_wrapper : String , pub mock_implementation : Option < String > , pub dao_policy : Option < String > , }
};
}
