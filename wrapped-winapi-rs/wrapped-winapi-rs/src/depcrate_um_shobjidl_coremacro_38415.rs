// Generated macro for macro_38415 (macro)
macro_rules! Depcrate_um_shobjidl_coremacro_38415 {
() => {
// Module: crate::um::shobjidl_core
// Provides: {"macro_38415"}
// Dependencies: {}
RIDL ! { # [uuid (0x000214ee , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IShellLinkA (IShellLinkAVtbl) : IUnknown (IUnknownVtbl) { fn GetPath (pszFile : LPSTR , cch : c_int , pfd : * mut WIN32_FIND_DATAA , fFlags : DWORD ,) -> HRESULT , fn GetIDList (ppidl : * mut PIDLIST_ABSOLUTE ,) -> HRESULT , fn SetIDList (pidl : PCIDLIST_ABSOLUTE ,) -> HRESULT , fn GetDescription (pszName : LPSTR , cch : c_int ,) -> HRESULT , fn SetDescription (pszName : LPCSTR ,) -> HRESULT , fn GetWorkingDirectory (pszDir : LPSTR , cch : c_int ,) -> HRESULT , fn SetWorkingDirectory (pszDir : LPCSTR ,) -> HRESULT , fn GetArguments (pszArgs : LPSTR , cch : c_int ,) -> HRESULT , fn SetArguments (pszArgs : LPCSTR ,) -> HRESULT , fn GetHotkey (pwHotkey : * mut WORD ,) -> HRESULT , fn SetHotkey (wHotkey : WORD ,) -> HRESULT , fn GetShowCmd (piShowCmd : * mut c_int ,) -> HRESULT , fn SetShowCmd (iShowCmd : c_int ,) -> HRESULT , fn GetIconLocation (pszIconPath : LPSTR , cch : c_int , piIcon : * mut c_int ,) -> HRESULT , fn SetIconLocation (pszIconPath : LPCSTR , iIcon : c_int ,) -> HRESULT , fn SetRelativePath (pszPathRel : LPCSTR , dwReserved : DWORD ,) -> HRESULT , fn Resolve (hwnd : HWND , fFlags : DWORD ,) -> HRESULT , fn SetPath (pszFile : LPCSTR ,) -> HRESULT , } }
};
}
