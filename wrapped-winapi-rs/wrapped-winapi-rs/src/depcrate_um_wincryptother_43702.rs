// Generated macro for other_43702 (other)
macro_rules! Depcrate_um_wincryptother_43702 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43702"}
// Dependencies: {}
extern "system" { pub fn CertCreateSelfSignCertificate (hCryptProvOrNCryptKey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE , pSubjectIssuerBlob : PCERT_NAME_BLOB , dwFlags : DWORD , pKeyProvInfo : PCRYPT_KEY_PROV_INFO , pSignatureAlgorithm : PCRYPT_ALGORITHM_IDENTIFIER , pStartTime : PSYSTEMTIME , pEndTime : PSYSTEMTIME , pExtensions : PCERT_EXTENSIONS ,) -> PCCERT_CONTEXT ; }
};
}
