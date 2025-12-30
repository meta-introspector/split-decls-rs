// Generated macro for other_43385 (other)
macro_rules! Depcrate_um_wincryptother_43385 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43385"}
// Dependencies: {}
extern "system" { pub fn CertGetEnhancedKeyUsage (pCertContext : PCCERT_CONTEXT , dwFlags : DWORD , pUsage : PCERT_ENHKEY_USAGE , pcbUsage : * mut DWORD ,) -> BOOL ; pub fn CertSetEnhancedKeyUsage (pCertContext : PCCERT_CONTEXT , pUsage : PCERT_ENHKEY_USAGE ,) -> BOOL ; pub fn CertAddEnhancedKeyUsageIdentifier (pCertContext : PCCERT_CONTEXT , pszUsageIdentifier : LPCSTR ,) -> BOOL ; pub fn CertRemoveEnhancedKeyUsageIdentifier (pCertContext : PCCERT_CONTEXT , pszUsageIdentifier : LPCSTR ,) -> BOOL ; pub fn CertGetValidUsages (cCerts : DWORD , rghCerts : * mut PCCERT_CONTEXT , cNumOIDs : * mut c_int , rghOIDs : * mut LPSTR , pcbOIDs : * mut DWORD ,) -> BOOL ; pub fn CryptMsgGetAndVerifySigner (hCryptMsg : HCRYPTMSG , cSignerStore : DWORD , rghSignerStore : * mut HCERTSTORE , dwFlags : DWORD , ppSigner : * mut PCCERT_CONTEXT , pdwSignerIndex : * mut DWORD ,) -> BOOL ; }
};
}
