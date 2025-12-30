// Generated macro for macro_39610 (macro)
macro_rules! Depcrate_um_wbemclimacro_39610 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39610"}
// Dependencies: {}
RIDL ! { # [uuid (0xeb87e1bc , 0x3233 , 0x11d2 , 0xae , 0xc9 , 0x00 , 0xc0 , 0x4f , 0xb6 , 0x88 , 0x20)] interface IWbemStatusCodeText (IWbemStatusCodeTextVtbl) : IUnknown (IUnknownVtbl) { fn GetErrorCodeText (hRes : HRESULT , LocaleId : LCID , lFlags : c_long , MessageText : * mut BSTR ,) -> HRESULT , fn GetFacilityCodeText (hRes : HRESULT , LocaleId : LCID , lFlags : c_long , MessageText : * mut BSTR ,) -> HRESULT , } }
};
}
