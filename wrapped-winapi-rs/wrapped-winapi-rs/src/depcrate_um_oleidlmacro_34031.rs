// Generated macro for macro_34031 (macro)
macro_rules! Depcrate_um_oleidlmacro_34031 {
() => {
// Module: crate::um::oleidl
// Provides: {"macro_34031"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000122 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IDropTarget (IDropTargetVtbl) : IUnknown (IUnknownVtbl) { fn DragEnter (pDataObj : * const IDataObject , grfKeyState : DWORD , pt : * const POINTL , pdwEffect : * mut DWORD ,) -> HRESULT , fn DragOver (grfKeyState : DWORD , pt : * const POINTL , pdwEffect : * mut DWORD ,) -> HRESULT , fn DragLeave () -> HRESULT , fn Drop (pDataObj : * const IDataObject , grfKeyState : DWORD , pt : * const POINTL , pdwEffect : * mut DWORD ,) -> HRESULT , } }
};
}
