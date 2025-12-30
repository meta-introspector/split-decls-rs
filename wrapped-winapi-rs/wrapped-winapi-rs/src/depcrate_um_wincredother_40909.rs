// Generated macro for other_40909 (other)
macro_rules! Depcrate_um_wincredother_40909 {
() => {
// Module: crate::um::wincred
// Provides: {"other_40909"}
// Dependencies: {}
extern "system" { pub fn CredEnumerateW (Filter : LPCWSTR , Flags : DWORD , Count : * mut DWORD , Credential : * mut * mut PCREDENTIALW ,) -> BOOL ; pub fn CredEnumerateA (Filter : LPCSTR , Flags : DWORD , Count : * mut DWORD , Credential : * mut * mut PCREDENTIALA ,) -> BOOL ; pub fn CredWriteDomainCredentialsW (TargetInfo : PCREDENTIAL_TARGET_INFORMATIONW , Credential : PCREDENTIALW , Flags : DWORD ,) -> BOOL ; pub fn CredWriteDomainCredentialsA (TargetInfo : PCREDENTIAL_TARGET_INFORMATIONA , Credential : PCREDENTIALA , Flags : DWORD ,) -> BOOL ; }
};
}
