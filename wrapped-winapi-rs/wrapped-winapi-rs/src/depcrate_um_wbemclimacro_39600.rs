// Generated macro for macro_39600 (macro)
macro_rules! Depcrate_um_wbemclimacro_39600 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39600"}
// Dependencies: {}
RIDL ! { # [uuid (0x027947e1 , 0xd731 , 0x11ce , 0xa3 , 0x57 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x01)] interface IEnumWbemClassObject (IEnumWbemClassObjectVtbl) : IUnknown (IUnknownVtbl) { fn Reset () -> HRESULT , fn Next (lTimeout : c_long , uCount : ULONG , apObjects : * mut * mut IWbemClassObject , puReturned : * mut ULONG ,) -> HRESULT , fn NextAsync (uCount : ULONG , pSink : * mut IWbemObjectSink ,) -> HRESULT , fn Clone (ppEnum : * mut * mut IEnumWbemClassObject ,) -> HRESULT , fn Skip (lTimeout : c_long , nCount : ULONG ,) -> HRESULT , } }
};
}
