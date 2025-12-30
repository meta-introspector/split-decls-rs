// Generated macro for other_40911 (other)
macro_rules! Depcrate_um_wincredother_40911 {
() => {
// Module: crate::um::wincred
// Provides: {"other_40911"}
// Dependencies: {}
extern "system" { pub fn CredReadDomainCredentialsW (TargetInfo : PCREDENTIAL_TARGET_INFORMATIONW , Flags : DWORD , Count : * mut DWORD , Credential : * mut * mut PCREDENTIALW ,) -> BOOL ; pub fn CredReadDomainCredentialsA (TargetInfo : PCREDENTIAL_TARGET_INFORMATIONA , Flags : DWORD , Count : * mut DWORD , Credential : * mut * mut PCREDENTIALA ,) -> BOOL ; pub fn CredDeleteW (TargetName : LPCWSTR , Type : DWORD , Flags : DWORD ,) -> BOOL ; pub fn CredDeleteA (TargetName : LPCSTR , Type : DWORD , Flags : DWORD ,) -> BOOL ; pub fn CredRenameW (OldTargetName : LPCWSTR , NewTargetName : LPCWSTR , Type : DWORD , Flags : DWORD ,) -> BOOL ; pub fn CredRenameA (OldTargetName : LPCSTR , NewTargetName : LPCSTR , Type : DWORD , Flags : DWORD ,) -> BOOL ; }
};
}
