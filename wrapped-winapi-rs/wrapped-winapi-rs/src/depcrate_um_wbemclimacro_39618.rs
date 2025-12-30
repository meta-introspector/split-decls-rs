// Generated macro for macro_39618 (macro)
macro_rules! Depcrate_um_wbemclimacro_39618 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39618"}
// Dependencies: {}
RIDL ! { # [uuid (0x2705c288 , 0x79ae , 0x11d2 , 0xb3 , 0x48 , 0x00 , 0x10 , 0x5a , 0x1f , 0x81 , 0x77)] interface IWbemHiPerfEnum (IWbemHiPerfEnumVtbl) : IUnknown (IUnknownVtbl) { fn AddObjects (lFlags : c_long , uNumObjects : ULONG , apIds : * mut c_long , apObj : * mut * mut IWbemObjectAccess ,) -> HRESULT , fn RemoveObjects (lFlags : c_long , uNumObjects : ULONG , apIds : * mut c_long ,) -> HRESULT , fn GetObjects (lFlags : c_long , uNumObjects : ULONG , apObj : * mut * mut IWbemObjectAccess , puReturned : * mut ULONG ,) -> HRESULT , fn RemoveAll (lFlags : c_long ,) -> HRESULT , } }
};
}
