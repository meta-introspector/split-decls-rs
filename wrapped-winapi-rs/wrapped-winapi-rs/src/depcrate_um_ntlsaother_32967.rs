// Generated macro for other_32967 (other)
macro_rules! Depcrate_um_ntlsaother_32967 {
() => {
// Module: crate::um::ntlsa
// Provides: {"other_32967"}
// Dependencies: {}
extern "system" { pub fn LsaFreeMemory (Buffer : PVOID ,) -> NTSTATUS ; pub fn LsaClose (ObjectHandle : LSA_HANDLE ,) -> NTSTATUS ; pub fn LsaDelete (ObjectHandle : LSA_HANDLE ,) -> NTSTATUS ; pub fn LsaQuerySecurityObject (ObjectHandle : LSA_HANDLE , SecurityInformation : SECURITY_INFORMATION , SecurityDescriptor : * mut PSECURITY_DESCRIPTOR ,) -> NTSTATUS ; pub fn LsaSetSecurityObject (ObjectHandle : LSA_HANDLE , SecurityInformation : SECURITY_INFORMATION , SecurityDescriptor : PSECURITY_DESCRIPTOR ,) -> NTSTATUS ; pub fn LsaChangePassword (ServerName : PLSA_UNICODE_STRING , DomainName : PLSA_UNICODE_STRING , AccountName : PLSA_UNICODE_STRING , OldPassword : PLSA_UNICODE_STRING , NewPassword : PLSA_UNICODE_STRING ,) -> NTSTATUS ; }
};
}
