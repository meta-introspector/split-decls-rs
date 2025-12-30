// Generated macro for other_32321 (other)
macro_rules! Depcrate_um_mssipother_32321 {
() => {
// Module: crate::um::mssip
// Provides: {"other_32321"}
// Dependencies: {}
extern "system" { pub fn CryptSIPPutSignedDataMsg (pSubjectInfo : * mut SIP_SUBJECTINFO , dwEncodingType : DWORD , pdwIndex : * mut DWORD , cbSignedDataMsg : DWORD , pbSignedDataMsg : * mut BYTE ,) -> BOOL ; }
};
}
