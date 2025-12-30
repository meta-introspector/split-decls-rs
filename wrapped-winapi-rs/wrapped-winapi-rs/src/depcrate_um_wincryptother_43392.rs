// Generated macro for other_43392 (other)
macro_rules! Depcrate_um_wincryptother_43392 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43392"}
// Dependencies: {}
extern "system" { pub fn CertFindSubjectInSortedCTL (pSubjectIdentifier : PCRYPT_DATA_BLOB , pCtlContext : PCCTL_CONTEXT , dwFlags : DWORD , pvReserved : * mut c_void , pEncodedAttributes : PCRYPT_DER_BLOB ,) -> BOOL ; pub fn CertEnumSubjectInSortedCTL (pCtlContext : PCCTL_CONTEXT , ppvNextSubject : * mut * mut c_void , pSubjectIdentifier : PCRYPT_DER_BLOB , pEncodedAttributes : PCRYPT_DER_BLOB ,) -> BOOL ; }
};
}
