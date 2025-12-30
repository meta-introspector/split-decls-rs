// Generated macro for other_43688 (other)
macro_rules! Depcrate_um_wincryptother_43688 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43688"}
// Dependencies: {}
extern "system" { pub fn CryptGetTimeValidObject (pszTimeValidOid : LPCSTR , pvPara : LPVOID , pIssuer : PCCERT_CONTEXT , pftValidFor : LPFILETIME , dwFlags : DWORD , dwTimeout : DWORD , ppvObject : * mut LPVOID , pCredentials : PCRYPT_CREDENTIALS , pExtraInfo : PCRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO ,) -> BOOL ; }
};
}
