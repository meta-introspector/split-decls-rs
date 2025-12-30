// Generated macro for other_41762 (other)
macro_rules! Depcrate_um_wincryptother_41762 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_41762"}
// Dependencies: {}
extern "system" { pub fn CryptEncodeObjectEx (dwCertEncodingType : DWORD , lpszStructType : LPCSTR , pvStructInfo : * const c_void , dwFlags : DWORD , pEncodePara : PCRYPT_ENCODE_PARA , pvEncoded : * mut c_void , pcbEncoded : * mut DWORD ,) -> BOOL ; pub fn CryptEncodeObject (dwCertEncodingType : DWORD , lpszStructType : LPCSTR , pvStructInfo : * const c_void , pbEncoded : * mut BYTE , pcbEncoded : * mut DWORD ,) -> BOOL ; }
};
}
