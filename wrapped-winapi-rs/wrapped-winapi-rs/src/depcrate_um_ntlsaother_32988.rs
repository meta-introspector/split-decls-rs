// Generated macro for other_32988 (other)
macro_rules! Depcrate_um_ntlsaother_32988 {
() => {
// Module: crate::um::ntlsa
// Provides: {"other_32988"}
// Dependencies: {}
extern "C" { pub fn LsaGetUserName (UserName : * mut PLSA_UNICODE_STRING , DomainName : * mut PLSA_UNICODE_STRING ,) -> NTSTATUS ; pub fn LsaGetRemoteUserName (SystemName : PLSA_UNICODE_STRING , UserName : * mut PLSA_UNICODE_STRING , DomainName : * mut PLSA_UNICODE_STRING ,) -> NTSTATUS ; }
};
}
