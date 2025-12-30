// Generated macro for macro_43997 (macro)
macro_rules! Depcrate_um_wincryptmacro_43997 {
() => {
// Module: crate::um::wincrypt
// Provides: {"macro_43997"}
// Dependencies: {}
STRUCT ! { struct CERT_CHAIN_FIND_ISSUER_PARA { cbSize : DWORD , pszUsageIdentifier : LPCSTR , dwKeySpec : DWORD , dwAcquirePrivateKeyFlags : DWORD , cIssuer : DWORD , rgIssuer : * mut CERT_NAME_BLOB , pfnFindCallback : PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK , pvFindArg : * mut c_void , pdwIssuerChainIndex : * mut DWORD , pdwIssuerElementIndex : * mut DWORD , } }
};
}
