// Generated macro for macro_38364 (macro)
macro_rules! Depcrate_um_shobjidlmacro_38364 {
() => {
// Module: crate::um::shobjidl
// Provides: {"macro_38364"}
// Dependencies: {}
RIDL ! { # [uuid (0x973510db , 0x7d7f , 0x452b , 0x89 , 0x75 , 0x74 , 0xa8 , 0x58 , 0x28 , 0xd3 , 0x54)] interface IFileDialogEvents (IFileDialogEventsVtbl) : IUnknown (IUnknownVtbl) { fn OnFileOk (pfd : * mut IFileDialog ,) -> HRESULT , fn OnFolderChanging (pfd : * mut IFileDialog , psiFolder : * mut IShellItem ,) -> HRESULT , fn OnFolderChange (pfd : * mut IFileDialog ,) -> HRESULT , fn OnSelectionChange (pfd : * mut IFileDialog ,) -> HRESULT , fn OnShareViolation (pfd : * mut IFileDialog , psi : * mut IShellItem , pResponse : * mut FDE_SHAREVIOLATION_RESPONSE ,) -> HRESULT , fn OnTypeChange (pfd : * mut IFileDialog ,) -> HRESULT , fn OnOverwrite (pfd : * mut IFileDialog , psi : * mut IShellItem , pResponse : * mut FDE_OVERWRITE_RESPONSE ,) -> HRESULT , } }
};
}
