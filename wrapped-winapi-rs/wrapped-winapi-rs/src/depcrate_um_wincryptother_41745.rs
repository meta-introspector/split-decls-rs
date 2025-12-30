// Generated macro for other_41745 (other)
macro_rules! Depcrate_um_wincryptother_41745 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_41745"}
// Dependencies: {}
extern "system" { pub fn CryptFormatObject (dwCertEncodingType : DWORD , dwFormatType : DWORD , dwFormatStrType : DWORD , pFormatStruct : * mut c_void , lpszStructType : LPCSTR , pbEncoded : * const BYTE , cbEncoded : DWORD , pbFormat : * mut c_void , pcbFormat : * mut DWORD ,) -> BOOL ; }
};
}
