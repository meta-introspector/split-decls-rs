// Generated macro for macro_10384 (macro)
macro_rules! Depcrate_shared_tcpmibmacro_10384 {
() => {
// Module: crate::shared::tcpmib
// Provides: {"macro_10384"}
// Dependencies: {}
STRUCT ! { struct MIB_TCP6ROW_OWNER_MODULE { ucLocalAddr : [UCHAR ; 16] , dwLocalScopeId : DWORD , dwLocalPort : DWORD , ucRemoteAddr : [UCHAR ; 16] , dwRemoteScopeId : DWORD , dwRemotePort : DWORD , dwState : DWORD , dwOwningPid : DWORD , liCreateTimestamp : LARGE_INTEGER , OwningModuleInfo : [ULONGLONG ; TCPIP_OWNING_MODULE_SIZE] , } }
};
}
