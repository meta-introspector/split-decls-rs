// Generated macro for other_43500 (other)
macro_rules! Depcrate_um_wincryptother_43500 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43500"}
// Dependencies: {}
extern "system" { pub fn CertStrToNameA (dwCertEncodingType : DWORD , pszX500 : LPCSTR , dwStrType : DWORD , pvReserved : * mut c_void , pbEncoded : * mut BYTE , pcbEncoded : * mut DWORD , ppszError : * mut LPCSTR ,) -> BOOL ; pub fn CertStrToNameW (dwCertEncodingType : DWORD , pszX500 : LPCWSTR , dwStrType : DWORD , pvReserved : * mut c_void , pbEncoded : * mut BYTE , pcbEncoded : * mut DWORD , ppszError : * mut LPCWSTR ,) -> BOOL ; pub fn CertGetNameStringA (pCertContext : PCCERT_CONTEXT , dwType : DWORD , dwFlags : DWORD , pvTypePara : * mut c_void , pszNameString : LPSTR , cchNameString : DWORD ,) -> DWORD ; pub fn CertGetNameStringW (pCertContext : PCCERT_CONTEXT , dwType : DWORD , dwFlags : DWORD , pvTypePara : * mut c_void , pszNameString : LPWSTR , cchNameString : DWORD ,) -> DWORD ; }
};
}
