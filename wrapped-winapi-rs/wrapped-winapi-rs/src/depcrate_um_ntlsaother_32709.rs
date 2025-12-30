// Generated macro for other_32709 (other)
macro_rules! Depcrate_um_ntlsaother_32709 {
() => {
// Module: crate::um::ntlsa
// Provides: {"other_32709"}
// Dependencies: {}
extern "system" { pub fn LsaRegisterLogonProcess (LogonProcessName : PLSA_STRING , LsaHandle : PHANDLE , SecurityMode : PLSA_OPERATIONAL_MODE ,) -> NTSTATUS ; pub fn LsaLogonUser (LsaHandle : HANDLE , OriginName : PLSA_STRING , LogonType : SECURITY_LOGON_TYPE , AuthenticationPackage : ULONG , AuthenticationInformation : PVOID , AuthenticationInformationLength : ULONG , LocalGroups : PTOKEN_GROUPS , SourceContext : PTOKEN_SOURCE , ProfileBuffer : * mut PVOID , ProfileBufferLength : PULONG , LogonId : PLUID , Token : PHANDLE , Quotas : PQUOTA_LIMITS , SubStatus : PNTSTATUS ,) -> NTSTATUS ; pub fn LsaLookupAuthenticationPackage (LsaHandle : HANDLE , PackageName : PLSA_STRING , AuthenticationPackage : PULONG ,) -> NTSTATUS ; pub fn LsaFreeReturnBuffer (Buffer : PVOID ,) -> NTSTATUS ; pub fn LsaCallAuthenticationPackage (LsaHandle : HANDLE , AuthenticationPackage : ULONG , ProtocolSubmitBuffer : PVOID , SubmitBufferLength : ULONG , ProtocolReturnBuffer : * mut PVOID , ReturnBufferLength : PULONG , ProtocolStatus : PNTSTATUS ,) -> NTSTATUS ; pub fn LsaDeregisterLogonProcess (LsaHandle : HANDLE ,) -> NTSTATUS ; pub fn LsaConnectUntrusted (LsaHandle : PHANDLE ,) -> NTSTATUS ; }
};
}
