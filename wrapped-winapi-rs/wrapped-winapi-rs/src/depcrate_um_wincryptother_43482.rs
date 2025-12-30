// Generated macro for other_43482 (other)
macro_rules! Depcrate_um_wincryptother_43482 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43482"}
// Dependencies: {}
extern "system" { pub fn CryptExportPKCS8 (hCryptProv : HCRYPTPROV , dwKeySpec : DWORD , pszPrivateKeyObjId : LPSTR , dwFlags : DWORD , pvAuxInfo : * mut c_void , pbPrivateKeyBlob : * mut BYTE , pcbPrivateKeyBlob : * mut DWORD ,) -> BOOL ; pub fn CryptExportPKCS8Ex (psExportParams : CRYPT_PKCS8_EXPORT_PARAMS , dwKeySpec : DWORD , pvAuxInfo : * mut c_void , pbPrivateKeyBlob : * mut BYTE , pcbPrivateKeyBlob : * mut DWORD ,) -> BOOL ; pub fn CryptHashPublicKeyInfo (hCryptProv : HCRYPTPROV_LEGACY , Algid : ALG_ID , dwFlags : DWORD , dwCertEncodingType : DWORD , pInfo : PCERT_PUBLIC_KEY_INFO , pbComputedHash : * mut BYTE , pcbComputedHash : * mut DWORD ,) -> BOOL ; pub fn CertRDNValueToStrA (dwValueType : DWORD , pValue : PCERT_RDN_VALUE_BLOB , psz : LPSTR , csz : DWORD ,) -> DWORD ; pub fn CertRDNValueToStrW (dwValueType : DWORD , pValue : PCERT_RDN_VALUE_BLOB , psz : LPWSTR , csz : DWORD ,) -> DWORD ; pub fn CertNameToStrA (dwCertEncodingType : DWORD , pName : PCERT_NAME_BLOB , dwStrType : DWORD , psz : LPSTR , csz : DWORD ,) -> DWORD ; pub fn CertNameToStrW (dwCertEncodingType : DWORD , pName : PCERT_NAME_BLOB , dwStrType : DWORD , psz : LPWSTR , csz : DWORD ,) -> DWORD ; }
};
}
