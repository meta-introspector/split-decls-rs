// Generated macro for other_43433 (other)
macro_rules! Depcrate_um_wincryptother_43433 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43433"}
// Dependencies: {}
extern "system" { pub fn CertIsStrongHashToSign (pStrongSignPara : PCCERT_STRONG_SIGN_PARA , pwszCNGHashAlgid : LPCWSTR , pSigningCert : PCCERT_CONTEXT ,) -> BOOL ; pub fn CryptHashToBeSigned (hCryptProv : HCRYPTPROV_LEGACY , dwCertEncodingType : DWORD , pbEncoded : * const BYTE , cbEncoded : DWORD , pbComputedHash : * mut BYTE , pcbComputedHash : * mut DWORD ,) -> BOOL ; pub fn CryptHashCertificate (hCryptProv : HCRYPTPROV_LEGACY , Algid : ALG_ID , dwFlags : DWORD , pbEncoded : * const BYTE , cbEncoded : DWORD , pbComputedHash : * mut BYTE , pcbComputedHash : * mut DWORD ,) -> BOOL ; pub fn CryptHashCertificate2 (pwszCNGHashAlgid : LPCWSTR , dwFlags : DWORD , pvReserved : * mut c_void , pbEncoded : * const BYTE , cbEncoded : DWORD , pbComputedHash : * mut BYTE , pcbComputedHash : * mut DWORD ,) -> BOOL ; pub fn CryptSignCertificate (hCryptProvOrNCryptKey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE , dwKeySpec : DWORD , dwCertEncodingType : DWORD , pbEncodedToBeSigned : * const BYTE , cbEncodedToBeSigned : DWORD , pSignatureAlgorithm : PCRYPT_ALGORITHM_IDENTIFIER , pvHashAuxInfo : * const c_void , pbSignature : * mut BYTE , pcbSignature : * mut DWORD ,) -> BOOL ; pub fn CryptSignAndEncodeCertificate (hCryptProvOrNCryptKey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE , dwKeySpec : DWORD , dwCertEncodingType : DWORD , lpszStructType : LPCSTR , pvStructInfo : * const c_void , pSignatureAlgorithm : PCRYPT_ALGORITHM_IDENTIFIER , pvHashAuxInfo : * const c_void , pbEncoded : * mut BYTE , pcbEncoded : * mut DWORD ,) -> BOOL ; }
};
}
