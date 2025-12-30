// Generated macro for other_32336 (other)
macro_rules! Depcrate_um_mssipother_32336 {
() => {
// Module: crate::um::mssip
// Provides: {"other_32336"}
// Dependencies: {}
extern "system" { pub fn CryptSIPLoad (pgSubject : * const GUID , dwFlags : DWORD , pSipDispatch : * mut SIP_DISPATCH_INFO ,) -> BOOL ; pub fn CryptSIPRetrieveSubjectGuid (FileName : LPCWSTR , hFileIn : HANDLE , pgSubject : * mut GUID ,) -> BOOL ; pub fn CryptSIPRetrieveSubjectGuidForCatalogFile (FileName : LPCWSTR , hFileIn : HANDLE , pgSubject : * mut GUID ,) -> BOOL ; pub fn CryptSIPAddProvider (psNewProv : * mut SIP_ADD_NEWPROVIDER ,) -> BOOL ; pub fn CryptSIPRemoveProvider (pgProv : * mut GUID ,) -> BOOL ; pub fn CryptSIPGetCaps (pSubjInfo : * mut SIP_SUBJECTINFO , pCaps : * mut SIP_CAP_SET ,) -> BOOL ; }
};
}
