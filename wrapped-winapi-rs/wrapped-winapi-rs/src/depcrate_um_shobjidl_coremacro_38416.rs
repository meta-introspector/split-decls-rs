// Generated macro for macro_38416 (macro)
macro_rules! Depcrate_um_shobjidl_coremacro_38416 {
() => {
// Module: crate::um::shobjidl_core
// Provides: {"macro_38416"}
// Dependencies: {}
RIDL ! { # [uuid (0x000214f9 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IShellLinkW (IShellLinkWVtbl) : IUnknown (IUnknownVtbl) { fn GetPath (pszFile : LPWSTR , cch : c_int , pfd : * mut WIN32_FIND_DATAW , fFlags : DWORD ,) -> HRESULT , fn GetIDList (ppidl : * mut PIDLIST_ABSOLUTE ,) -> HRESULT , fn SetIDList (pidl : PCIDLIST_ABSOLUTE ,) -> HRESULT , fn GetDescription (pszName : LPWSTR , cch : c_int ,) -> HRESULT , fn SetDescription (pszName : LPCWSTR ,) -> HRESULT , fn GetWorkingDirectory (pszDir : LPWSTR , cch : c_int ,) -> HRESULT , fn SetWorkingDirectory (pszDir : LPCWSTR ,) -> HRESULT , fn GetArguments (pszArgs : LPWSTR , cch : c_int ,) -> HRESULT , fn SetArguments (pszArgs : LPCWSTR ,) -> HRESULT , fn GetHotkey (pwHotkey : * mut WORD ,) -> HRESULT , fn SetHotkey (wHotkey : WORD ,) -> HRESULT , fn GetShowCmd (piShowCmd : * mut c_int ,) -> HRESULT , fn SetShowCmd (iShowCmd : c_int ,) -> HRESULT , fn GetIconLocation (pszIconPath : LPWSTR , cch : c_int , piIcon : * mut c_int ,) -> HRESULT , fn SetIconLocation (pszIconPath : LPCWSTR , iIcon : c_int ,) -> HRESULT , fn SetRelativePath (pszPathRel : LPCWSTR , dwReserved : DWORD ,) -> HRESULT , fn Resolve (hwnd : HWND , fFlags : DWORD ,) -> HRESULT , fn SetPath (pszFile : LPCWSTR ,) -> HRESULT , } }
};
}
