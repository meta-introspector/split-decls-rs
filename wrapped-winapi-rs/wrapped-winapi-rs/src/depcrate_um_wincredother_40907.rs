// Generated macro for other_40907 (other)
macro_rules! Depcrate_um_wincredother_40907 {
() => {
// Module: crate::um::wincred
// Provides: {"other_40907"}
// Dependencies: {}
extern "system" { pub fn CredWriteW (Credential : PCREDENTIALW , Flags : DWORD ,) -> BOOL ; pub fn CredWriteA (Credential : PCREDENTIALA , Flags : DWORD ,) -> BOOL ; pub fn CredReadW (TargetName : LPCWSTR , Type : DWORD , Flags : DWORD , Credential : * mut PCREDENTIALW ,) -> BOOL ; pub fn CredReadA (TargetName : LPCSTR , Type : DWORD , Flags : DWORD , Credential : * mut PCREDENTIALA ,) -> BOOL ; }
};
}
