// Generated macro for other_42761 (other)
macro_rules! Depcrate_um_wincryptother_42761 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_42761"}
// Dependencies: {}
extern "system" { pub fn CryptMsgCountersign (hCryptMsg : HCRYPTMSG , dwIndex : DWORD , cCountersigners : DWORD , rgCountersigners : PCMSG_SIGNER_ENCODE_INFO ,) -> BOOL ; pub fn CryptMsgCountersignEncoded (dwEncodingType : DWORD , pbSignerInfo : PBYTE , cbSignerInfo : DWORD , cCountersigners : DWORD , rgCountersigners : PCMSG_SIGNER_ENCODE_INFO , pbCountersignature : PBYTE , pcbCountersignature : PDWORD ,) -> BOOL ; }
};
}
