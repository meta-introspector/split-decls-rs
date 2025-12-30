// Generated macro for other_38349 (other)
macro_rules! Depcrate_um_shlobjother_38349 {
() => {
// Module: crate::um::shlobj
// Provides: {"other_38349"}
// Dependencies: {}
extern "system" { pub fn SHGetKnownFolderIDList (rfid : REFKNOWNFOLDERID , dwFlags : DWORD , hToken : HANDLE , ppidl : * mut PIDLIST_ABSOLUTE ,) -> HRESULT ; pub fn SHSetKnownFolderPath (rfid : REFKNOWNFOLDERID , dwFlags : DWORD , hToken : HANDLE , pszPath : PCWSTR ,) -> HRESULT ; pub fn SHGetKnownFolderPath (rfid : REFKNOWNFOLDERID , dwFlags : DWORD , hToken : HANDLE , pszPath : * mut PWSTR ,) -> HRESULT ; pub fn SHGetKnownFolderItem (rfid : REFKNOWNFOLDERID , flags : KNOWN_FOLDER_FLAG , hToken : HANDLE , riid : REFIID , ppv : * mut * mut c_void ,) -> HRESULT ; }
};
}
