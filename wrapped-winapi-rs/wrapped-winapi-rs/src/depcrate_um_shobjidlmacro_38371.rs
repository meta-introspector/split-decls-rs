// Generated macro for macro_38371 (macro)
macro_rules! Depcrate_um_shobjidlmacro_38371 {
() => {
// Module: crate::um::shobjidl
// Provides: {"macro_38371"}
// Dependencies: {}
RIDL ! { # [uuid (0x36116642 , 0xd713 , 0x4b97 , 0x9b , 0x83 , 0x74 , 0x84 , 0xa9 , 0xd0 , 0x04 , 0x33)] interface IFileDialogControlEvents (IFileDialogControlEventsVtbl) : IUnknown (IUnknownVtbl) { fn OnItemSelected (pfdc : * mut IFileDialogCustomize , dwIDCtl : DWORD , dwIDItem : DWORD ,) -> HRESULT , fn OnButtonClicked (pfdc : * mut IFileDialogCustomize , dwIDCtl : DWORD ,) -> HRESULT , fn OnCheckButtonToggled (pfdc : * mut IFileDialogCustomize , dwIDCtl : DWORD , bChecked : BOOL ,) -> HRESULT , fn OnControlActivating (pfdc : * mut IFileDialogCustomize , dwIDCtl : DWORD ,) -> HRESULT , } }
};
}
