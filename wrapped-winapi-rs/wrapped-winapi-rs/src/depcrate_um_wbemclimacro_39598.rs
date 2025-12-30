// Generated macro for macro_39598 (macro)
macro_rules! Depcrate_um_wbemclimacro_39598 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39598"}
// Dependencies: {}
RIDL ! { # [uuid (0x7c857801 , 0x7381 , 0x11cf , 0x88 , 0x4d , 0x00 , 0xaa , 0x00 , 0x4b , 0x2e , 0x24)] interface IWbemObjectSink (IWbemObjectSinkVtbl) : IUnknown (IUnknownVtbl) { fn Indicate (lObjectCount : c_long , apObjArray : * mut * mut IWbemClassObject ,) -> HRESULT , fn SetStatus (lFlags : c_long , hResult : HRESULT , strParam : BSTR , pObjParam : * mut IWbemClassObject ,) -> HRESULT , } }
};
}
