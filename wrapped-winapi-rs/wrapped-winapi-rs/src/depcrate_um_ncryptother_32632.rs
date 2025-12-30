// Generated macro for other_32632 (other)
macro_rules! Depcrate_um_ncryptother_32632 {
() => {
// Module: crate::um::ncrypt
// Provides: {"other_32632"}
// Dependencies: {}
extern "system" { pub fn NCryptSetProperty (hObject : NCRYPT_HANDLE , pszProperty : LPCWSTR , pbInput : PBYTE , cbInput : DWORD , dwFlags : DWORD ,) -> SECURITY_STATUS ; pub fn NCryptImportKey (hProvider : NCRYPT_PROV_HANDLE , hImportKey : NCRYPT_KEY_HANDLE , pszBlobType : LPCWSTR , pParameterList : * const NCryptBufferDesc , phKey : * mut NCRYPT_KEY_HANDLE , pbData : PBYTE , cbData : DWORD , dwFlags : DWORD ,) -> SECURITY_STATUS ; pub fn NCryptFreeObject (hObject : NCRYPT_HANDLE ,) -> SECURITY_STATUS ; }
};
}
