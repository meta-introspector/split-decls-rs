// Generated macro for macro_39637 (macro)
macro_rules! Depcrate_um_wbemclimacro_39637 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39637"}
// Dependencies: {}
RIDL ! { # [uuid (0xbfbf883a , 0xcad7 , 0x11d3 , 0xa1 , 0x1b , 0x00 , 0x10 , 0x5a , 0x1f , 0x51 , 0x5a)] interface IWbemObjectTextSrc (IWbemObjectTextSrcVtbl) : IUnknown (IUnknownVtbl) { fn GetText (lFlags : c_long , pObj : * mut IWbemClassObject , uObjTextFormat : ULONG , pCtx : * mut IWbemContext , strText : * mut BSTR ,) -> HRESULT , fn CreateFromText (lFlags : c_long , strText : BSTR , uObjTextFormat : ULONG , pCtx : * mut IWbemContext , pNewObj : * mut * mut IWbemClassObject ,) -> HRESULT , } }
};
}
