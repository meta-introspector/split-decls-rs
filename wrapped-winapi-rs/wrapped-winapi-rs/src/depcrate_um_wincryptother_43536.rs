// Generated macro for other_43536 (other)
macro_rules! Depcrate_um_wincryptother_43536 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43536"}
// Dependencies: {}
extern "system" { pub fn FindCertsByIssuer (pCertChains : PCERT_CHAIN , pcbCertChains : * mut DWORD , pcCertChains : * mut DWORD , pbEncodedIssuerName : * mut BYTE , cbEncodedIssuerName : DWORD , pwszPurpose : LPCWSTR , dwKeySpec : DWORD ,) -> HRESULT ; pub fn CryptQueryObject (dwObjectType : DWORD , pvObject : * const c_void , dwExpectedContentTypeFlags : DWORD , dwExpectedFormatTypeFlags : DWORD , dwFlags : DWORD , pdwMsgAndCertEncodingType : * mut DWORD , pdwContentType : * mut DWORD , pdwFormatType : * mut DWORD , phCertStore : * mut HCERTSTORE , phMsg : * mut HCRYPTMSG , ppvContext : * mut * const c_void ,) -> BOOL ; }
};
}
