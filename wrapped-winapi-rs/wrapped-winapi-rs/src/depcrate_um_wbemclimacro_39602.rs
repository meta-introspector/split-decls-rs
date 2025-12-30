// Generated macro for macro_39602 (macro)
macro_rules! Depcrate_um_wbemclimacro_39602 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39602"}
// Dependencies: {}
RIDL ! { # [uuid (0x44aca675 , 0xe8fc , 0x11d0 , 0xa0 , 0x7c , 0x00 , 0xc0 , 0x4f , 0xb6 , 0x88 , 0x20)] interface IWbemCallResult (IWbemCallResultVtbl) : IUnknown (IUnknownVtbl) { fn GetResultObject (lTimeout : c_long , ppResultObject : * mut * mut IWbemClassObject ,) -> HRESULT , fn GetResultString (lTimeout : c_long , pstrResultString : * mut BSTR ,) -> HRESULT , fn GetResultServices (lTimeout : c_long , ppServices : * mut * mut IWbemServices ,) -> HRESULT , fn GetCallStatus (lTimeout : c_long , plStatus : * mut c_long ,) -> HRESULT , } }
};
}
