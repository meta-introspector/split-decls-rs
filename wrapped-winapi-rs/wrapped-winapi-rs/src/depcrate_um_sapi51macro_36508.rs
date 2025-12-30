// Generated macro for macro_36508 (macro)
macro_rules! Depcrate_um_sapi51macro_36508 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36508"}
// Dependencies: {}
RIDL ! { # [uuid (0x5b4fb971 , 0xb115 , 0x4de1 , 0xad , 0x97 , 0xe4 , 0x82 , 0xe3 , 0xbf , 0x6e , 0xe4)] interface ISpProperties (ISpPropertiesVtbl) : IUnknown (IUnknownVtbl) { fn SetPropertyNum (pName : LPCWSTR , lValue : LONG ,) -> HRESULT , fn GetPropertyNum (pName : LPCWSTR , plValue : * mut LONG ,) -> HRESULT , fn SetPropertyString (pName : LPCWSTR , pValue : LPCWSTR ,) -> HRESULT , fn GetPropertyString (pName : LPCWSTR , ppCoMemValue : * mut LPWSTR ,) -> HRESULT , } }
};
}
