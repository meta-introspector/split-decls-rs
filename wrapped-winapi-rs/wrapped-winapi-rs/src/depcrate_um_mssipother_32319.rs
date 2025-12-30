// Generated macro for other_32319 (other)
macro_rules! Depcrate_um_mssipother_32319 {
() => {
// Module: crate::um::mssip
// Provides: {"other_32319"}
// Dependencies: {}
extern "system" { pub fn CryptSIPGetSignedDataMsg (pSubjectInfo : * mut SIP_SUBJECTINFO , pdwEncodingType : * mut DWORD , dwIndex : DWORD , pcbSignedDataMsg : * mut DWORD , pbSignedDataMsg : * mut BYTE ,) -> BOOL ; }
};
}
