// Generated macro for macro_33828 (macro)
macro_rules! Depcrate_um_objidlmacro_33828 {
() => {
// Module: crate::um::objidl
// Provides: {"macro_33828"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000010 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IRunningObjectTable (IRunningObjectTableVtbl) : IUnknown (IUnknownVtbl) { fn Register (grfFlags : DWORD , punkObject : * mut IUnknown , pmkObjectName : * mut IMoniker , pdwRegister : * mut DWORD ,) -> HRESULT , fn Revoke (dwRegister : DWORD ,) -> HRESULT , fn IsRunning (pmkObjectName : * mut IMoniker ,) -> HRESULT , fn GetObject (pmkObjectName : * mut IMoniker , ppunkObject : * mut * mut IUnknown ,) -> HRESULT , fn NoteChangeTime (dwRegister : DWORD , pfiletime : * mut FILETIME ,) -> HRESULT , fn GetTimeOfLastChange (pmkObjectName : * mut IMoniker , pfiletime : * mut FILETIME ,) -> HRESULT , fn EnumRunning (ppenumMoniker : * mut * mut IEnumMoniker ,) -> HRESULT , } }
};
}
