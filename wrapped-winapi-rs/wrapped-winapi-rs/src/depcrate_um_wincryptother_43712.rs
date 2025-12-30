// Generated macro for other_43712 (other)
macro_rules! Depcrate_um_wincryptother_43712 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43712"}
// Dependencies: {}
extern "system" { pub fn CryptEnumKeyIdentifierProperties (pKeyIdentifier : * const CRYPT_HASH_BLOB , dwPropId : DWORD , dwFlags : DWORD , pwszComputerName : LPCWSTR , pvReserved : * mut c_void , pvArg : * mut c_void , pfnEnum : PFN_CRYPT_ENUM_KEYID_PROP ,) -> BOOL ; pub fn CryptCreateKeyIdentifierFromCSP (dwCertEncodingType : DWORD , pszPubKeyOID : LPCSTR , pPubKeyStruc : * const PUBLICKEYSTRUC , cbPubKeyStruc : DWORD , dwFlags : DWORD , pvReserved : * mut c_void , pbHash : * mut BYTE , pcbHash : * mut DWORD ,) -> BOOL ; }
};
}
