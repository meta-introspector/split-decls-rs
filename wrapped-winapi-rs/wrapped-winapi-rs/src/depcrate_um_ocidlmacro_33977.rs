// Generated macro for macro_33977 (macro)
macro_rules! Depcrate_um_ocidlmacro_33977 {
() => {
// Module: crate::um::ocidl
// Provides: {"macro_33977"}
// Dependencies: {}
RIDL ! { # [uuid (0x22f55882 , 0x280b , 0x11d0 , 0xa8 , 0xa9 , 0x00 , 0xa0 , 0xc9 , 0x0c , 0x20 , 0x04)] interface IPropertyBag2 (IPropertyBag2Vtbl) : IUnknown (IUnknownVtbl) { fn Read (cProperties : ULONG , pPropBag : * const PROPBAG2 , pErrLog : * const IErrorLog , pvarValue : * mut VARIANT , phrError : * mut HRESULT ,) -> HRESULT , fn Write (cProperties : ULONG , pPropBag : * const PROPBAG2 , pvarValue : * const VARIANT ,) -> HRESULT , fn CountProperties (pcProperties : * mut ULONG ,) -> HRESULT , fn GetPropertyInfo (iProperty : ULONG , cProperties : ULONG , pPropBag : * mut PROPBAG2 , pcProperties : * mut ULONG ,) -> HRESULT , fn LoadObject (pstrName : LPCOLESTR , dwHint : DWORD , pUnkObject : * const IUnknown , pErrLog : * const IErrorLog ,) -> HRESULT , } }
};
}
