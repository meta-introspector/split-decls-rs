// Generated macro for other_10291 (other)
macro_rules! Depcrate_shared_sspiother_10291 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10291"}
// Dependencies: {}
extern "system" { pub fn SspiUnmarshalCredUIContext (MarshaledCredUIContext : PUCHAR , MarshaledCredUIContextLength : ULONG , CredUIContext : * mut PSEC_WINNT_CREDUI_CONTEXT ,) -> SECURITY_STATUS ; pub fn SspiIsPromptingNeeded (ErrorOrNtStatus : c_ulong ,) -> BOOLEAN ; }
};
}
