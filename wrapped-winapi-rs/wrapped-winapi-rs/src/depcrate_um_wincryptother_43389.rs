// Generated macro for other_43389 (other)
macro_rules! Depcrate_um_wincryptother_43389 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43389"}
// Dependencies: {}
extern "system" { pub fn CryptMsgSignCTL (dwMsgEncodingType : DWORD , pbCtlContent : * mut BYTE , cbCtlContent : DWORD , pSignInfo : PCMSG_SIGNED_ENCODE_INFO , dwFlags : DWORD , pbEncoded : * mut BYTE , pcbEncoded : * mut DWORD ,) -> BOOL ; pub fn CryptMsgEncodeAndSignCTL (dwMsgEncodingType : DWORD , pCtlInfo : PCTL_INFO , pSignInfo : PCMSG_SIGNED_ENCODE_INFO , dwFlags : DWORD , pbEncoded : * mut BYTE , pcbEncoded : * mut DWORD ,) -> BOOL ; }
};
}
