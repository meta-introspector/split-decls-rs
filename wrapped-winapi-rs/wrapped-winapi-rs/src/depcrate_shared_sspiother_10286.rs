// Generated macro for other_10286 (other)
macro_rules! Depcrate_shared_sspiother_10286 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10286"}
// Dependencies: {}
extern "system" { pub fn SspiGetCredUIContext (ContextHandle : HANDLE , CredType : * mut GUID , LogonId : * mut LUID , CredUIContexts : * mut PSEC_WINNT_CREDUI_CONTEXT_VECTOR , TokenHandle : * mut HANDLE ,) -> SECURITY_STATUS ; pub fn SspiUpdateCredentials (ContextHandle : HANDLE , CredType : * mut GUID , FlatCredUIContextLength : ULONG , FlatCredUIContext : PUCHAR ,) -> SECURITY_STATUS ; }
};
}
