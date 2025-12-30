// Generated macro for macro_38366 (macro)
macro_rules! Depcrate_um_shobjidlmacro_38366 {
() => {
// Module: crate::um::shobjidl
// Provides: {"macro_38366"}
// Dependencies: {}
RIDL ! { # [uuid (0x42f85136 , 0xdb7e , 0x439c , 0x85 , 0xf1 , 0xe4 , 0x07 , 0x5d , 0x13 , 0x5f , 0xc8)] interface IFileDialog (IFileDialogVtbl) : IModalWindow (IModalWindowVtbl) { fn SetFileTypes (cFileTypes : UINT , rgFilterSpec : * const COMDLG_FILTERSPEC ,) -> HRESULT , fn SetFileTypeIndex (iFileType : UINT ,) -> HRESULT , fn GetFileTypeIndex (piFileType : * mut UINT ,) -> HRESULT , fn Advise (pfde : * mut IFileDialogEvents , pdwCookie : * mut DWORD ,) -> HRESULT , fn Unadvise (dwCookie : DWORD ,) -> HRESULT , fn SetOptions (fos : FILEOPENDIALOGOPTIONS ,) -> HRESULT , fn GetOptions (pfos : * mut FILEOPENDIALOGOPTIONS ,) -> HRESULT , fn SetDefaultFolder (psi : * mut IShellItem ,) -> HRESULT , fn SetFolder (psi : * mut IShellItem ,) -> HRESULT , fn GetFolder (ppsi : * mut * mut IShellItem ,) -> HRESULT , fn GetCurrentSelection (ppsi : * mut * mut IShellItem ,) -> HRESULT , fn SetFileName (pszName : LPCWSTR ,) -> HRESULT , fn GetFileName (pszName : * mut LPWSTR ,) -> HRESULT , fn SetTitle (pszTitle : LPCWSTR ,) -> HRESULT , fn SetOkButtonLabel (pszText : LPCWSTR ,) -> HRESULT , fn SetFileNameLabel (pszLabel : LPCWSTR ,) -> HRESULT , fn GetResult (ppsi : * mut * mut IShellItem ,) -> HRESULT , fn AddPlace (psi : * mut IShellItem , fdap : FDAP ,) -> HRESULT , fn SetDefaultExtension (pszDefaultExtension : LPCWSTR ,) -> HRESULT , fn Close (hr : HRESULT ,) -> HRESULT , fn SetClientGuid (guid : REFGUID ,) -> HRESULT , fn ClearClientData () -> HRESULT , fn SetFilter (pFilter : * mut IShellItemFilter ,) -> HRESULT , } }
};
}
