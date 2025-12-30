// Generated macro for other_42759 (other)
macro_rules! Depcrate_um_wincryptother_42759 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_42759"}
// Dependencies: {}
extern "system" { pub fn CryptMsgVerifyCountersignatureEncoded (hCryptProv : HCRYPTPROV_LEGACY , dwEncodingType : DWORD , pbSignerInfo : PBYTE , cbSignerInfo : DWORD , pbSignerInfoCountersignature : PBYTE , cbSignerInfoCountersignature : DWORD , pciCountersigner : PCERT_INFO ,) -> BOOL ; pub fn CryptMsgVerifyCountersignatureEncodedEx (hCryptProv : HCRYPTPROV_LEGACY , dwEncodingType : DWORD , pbSignerInfo : PBYTE , cbSignerInfo : DWORD , pbSignerInfoCountersignature : PBYTE , cbSignerInfoCountersignature : DWORD , dwSignerType : DWORD , pvSigner : * mut c_void , dwFlags : DWORD , pvExtra : * mut c_void ,) -> BOOL ; }
};
}
