// Generated macro for other_32972 (other)
macro_rules! Depcrate_um_ntlsaother_32972 {
() => {
// Module: crate::um::ntlsa
// Provides: {"other_32972"}
// Dependencies: {}
extern "system" { pub fn LsaEnumerateLogonSessions (LogonSessionCount : PULONG , LogonSessionList : * mut PLUID ,) -> NTSTATUS ; pub fn LsaGetLogonSessionData (LogonId : PLUID , ppLogonSessionData : * mut PSECURITY_LOGON_SESSION_DATA ,) -> NTSTATUS ; pub fn LsaOpenPolicy (SystemName : PLSA_UNICODE_STRING , ObjectAttributes : PLSA_OBJECT_ATTRIBUTES , DesiredAccess : ACCESS_MASK , PolicyHandle : PLSA_HANDLE ,) -> NTSTATUS ; pub fn LsaOpenPolicySce (SystemName : PLSA_UNICODE_STRING , ObjectAttributes : PLSA_OBJECT_ATTRIBUTES , DesiredAccess : ACCESS_MASK , PolicyHandle : PLSA_HANDLE ,) -> NTSTATUS ; }
};
}
