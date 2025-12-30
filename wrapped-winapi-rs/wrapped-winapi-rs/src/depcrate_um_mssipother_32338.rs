// Generated macro for other_32338 (other)
macro_rules! Depcrate_um_mssipother_32338 {
() => {
// Module: crate::um::mssip
// Provides: {"other_32338"}
// Dependencies: {}
extern "system" { pub fn CryptSIPGetSealedDigest (pSubjectInfo : * mut SIP_SUBJECTINFO , pSig : * const BYTE , dwSig : DWORD , pbDigest : * mut BYTE , pcbDigest : * mut DWORD ,) -> BOOL ; }
};
}
