// Generated macro for other_41775 (other)
macro_rules! Depcrate_um_wincryptother_41775 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_41775"}
// Dependencies: {}
extern "system" { pub fn CryptDecodeObjectEx (dwCertEncodingType : DWORD , lpszStructType : LPCSTR , pbEncoded : * const BYTE , cbEncoded : DWORD , dwFlags : DWORD , pDecodePara : PCRYPT_DECODE_PARA , pvStructInfo : * mut c_void , pcbStructInfo : * mut DWORD ,) -> BOOL ; pub fn CryptDecodeObject (dwCertEncodingType : DWORD , lpszStructType : LPCSTR , pbEncoded : * const BYTE , cbEncoded : DWORD , dwFlags : DWORD , pvStructInfo : * mut c_void , pcbStructInfo : * mut DWORD ,) -> BOOL ; }
};
}
