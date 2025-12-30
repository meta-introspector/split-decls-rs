// Generated macro for macro_33926 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33926 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33926"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000146 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IGlobalInterfaceTable (IGlobalInterfaceTableVtbl) : IUnknown (IUnknownVtbl) { fn RegisterInterfaceInGlobal (pUnk : * mut IUnknown , riid : REFIID , pdwCookie : * mut DWORD ,) -> HRESULT , fn RevokeInterfaceFromGlobal (dwCookie : DWORD ,) -> HRESULT , fn GetInterfaceFromGlobal (dwCookie : DWORD , riid : REFIID , ppv : * mut * mut c_void ,) -> HRESULT , } }
};
}
