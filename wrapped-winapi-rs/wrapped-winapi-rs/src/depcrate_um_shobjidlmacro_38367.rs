// Generated macro for macro_38367 (macro)
macro_rules! Depcrate_um_shobjidlmacro_38367 {
() => {
// Module: crate::um::shobjidl
// Provides: {"macro_38367"}
// Dependencies: {}
RIDL ! { # [uuid (0x84bccd23 , 0x5fde , 0x4cdb , 0xae , 0xa4 , 0xaf , 0x64 , 0xb8 , 0x3d , 0x78 , 0xab)] interface IFileSaveDialog (IFileSaveDialogVtbl) : IFileDialog (IFileDialogVtbl) { fn SetSaveAsItem (psi : * mut IShellItem ,) -> HRESULT , fn SetProperties (pStore : * mut IPropertyStore ,) -> HRESULT , fn SetCollectedProperties (pList : * mut IPropertyDescriptionList , fAppendDefault : BOOL ,) -> HRESULT , fn GetProperties (ppStore : * mut * mut IPropertyStore ,) -> HRESULT , fn ApplyProperties (psi : * mut IShellItem , pStore : * mut IPropertyStore , hwnd : HWND , pSink : * mut IFileOperationProgressSink ,) -> HRESULT , } }
};
}
