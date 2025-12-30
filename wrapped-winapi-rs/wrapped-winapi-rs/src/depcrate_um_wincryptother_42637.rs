// Generated macro for other_42637 (other)
macro_rules! Depcrate_um_wincryptother_42637 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_42637"}
// Dependencies: {}
extern "system" { pub fn CryptMsgOpenToEncode (dwMsgEncodingType : DWORD , dwFlags : DWORD , dwMsgType : DWORD , pvMsgEncodeInfo : * mut c_void , pszInnerContentObjID : LPSTR , pStreamInfo : PCMSG_STREAM_INFO ,) -> HCRYPTMSG ; pub fn CryptMsgCalculateEncodedLength (dwMsgEncodingType : DWORD , dwFlags : DWORD , dwMsgType : DWORD , pvMsgEncodeInfo : * const c_void , pszInnerContentObjID : LPSTR , cbData : DWORD ,) -> DWORD ; pub fn CryptMsgOpenToDecode (dwMsgEncodingType : DWORD , dwFlags : DWORD , dwMsgType : DWORD , hCryptProv : HCRYPTPROV_LEGACY , pRecipientInfo : PCERT_INFO , pStreamInfo : PCMSG_STREAM_INFO ,) -> HCRYPTMSG ; pub fn CryptMsgDuplicate (hCryptMsg : HCRYPTMSG ,) -> HCRYPTMSG ; pub fn CryptMsgClose (hCryptMsg : HCRYPTMSG ,) -> BOOL ; pub fn CryptMsgUpdate (hCryptMsg : HCRYPTMSG , pbData : * const BYTE , cbData : DWORD , fFinal : BOOL ,) -> BOOL ; pub fn CryptMsgGetParam (hCryptMsg : HCRYPTMSG , dwParamType : DWORD , dwIndex : DWORD , pvData : * mut c_void , pcbData : * mut DWORD ,) -> BOOL ; }
};
}
