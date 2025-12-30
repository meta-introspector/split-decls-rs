// Generated macro for other_43417 (other)
macro_rules! Depcrate_um_wincryptother_43417 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43417"}
// Dependencies: {}
extern "system" { pub fn CertCompareIntegerBlob (pInt1 : PCRYPT_INTEGER_BLOB , pInt2 : PCRYPT_INTEGER_BLOB ,) -> BOOL ; pub fn CertCompareCertificate (dwCertEncodingType : DWORD , pCertId1 : PCERT_INFO , pCertId2 : PCERT_INFO ,) -> BOOL ; pub fn CertCompareCertificateName (dwCertEncodingType : DWORD , pCertName1 : PCERT_NAME_BLOB , pCertName2 : PCERT_NAME_BLOB ,) -> BOOL ; pub fn CertIsRDNAttrsInCertificateName (dwCertEncodingType : DWORD , dwFlags : DWORD , pCertName : PCERT_NAME_BLOB , pRDN : PCERT_RDN ,) -> BOOL ; pub fn CertComparePublicKeyInfo (dwCertEncodingType : DWORD , pPublicKey1 : PCERT_PUBLIC_KEY_INFO , pPublicKey2 : PCERT_PUBLIC_KEY_INFO ,) -> BOOL ; pub fn CertGetPublicKeyLength (dwCertEncodingType : DWORD , pPublicKey : PCERT_PUBLIC_KEY_INFO ,) -> DWORD ; pub fn CryptVerifyCertificateSignature (hCryptProv : HCRYPTPROV_LEGACY , dwCertEncodingType : DWORD , pbEncoded : * const BYTE , cbEncoded : DWORD , pPublicKey : PCERT_PUBLIC_KEY_INFO ,) -> BOOL ; pub fn CryptVerifyCertificateSignatureEx (hCryptProv : HCRYPTPROV_LEGACY , dwCertEncodingType : DWORD , dwSubjectType : DWORD , pvSubject : * mut c_void , dwIssuerType : DWORD , pvIssuer : * mut c_void , dwFlags : DWORD , pvExtra : * mut c_void ,) -> BOOL ; }
};
}
