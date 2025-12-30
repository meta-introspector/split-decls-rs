// Generated macro for macro_33826 (macro)
macro_rules! Depcrate_um_objidlmacro_33826 {
() => {
// Module: crate::um::objidl
// Provides: {"macro_33826"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000000e , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IBindCtx (IBindCtxVtbl) : IUnknown (IUnknownVtbl) { fn RegisterObjectBound (punk : * mut IUnknown ,) -> HRESULT , fn RevokeObjectBound (punk : * mut IUnknown ,) -> HRESULT , fn ReleaseBoundObjects () -> HRESULT , fn SetBindOptions (pbindopts : * mut BIND_OPTS ,) -> HRESULT , fn GetBindOptions (pbindopts : * mut BIND_OPTS ,) -> HRESULT , fn GetRunningObjectTable (pprot : * mut * mut IRunningObjectTable ,) -> HRESULT , fn RegisterObjectParam (pszKey : LPOLESTR , punk : * mut IUnknown ,) -> HRESULT , fn GetObjectParam (pszKey : LPOLESTR , ppunk : * mut * mut IUnknown ,) -> HRESULT , fn EnumObjectParam (ppenum : * mut * mut IEnumString ,) -> HRESULT , fn RevokeObjectParam (pszKey : LPOLESTR ,) -> HRESULT , } }
};
}
